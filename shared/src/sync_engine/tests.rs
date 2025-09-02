use any_spawner::Executor;
use futures::{
    channel::mpsc,
    future::{select, Either},
    SinkExt, StreamExt,
};
use github_api::models::{IssuesCreateRequest, IssuesCreateRequestTitle};
use github_webhook_body::{Issues, IssuesOpenedIssue, SomethingWithAnId, WebhookBody};
use jiff::Timestamp;
use maplit::{hashmap, hashset};
use mockall::predicate;
use parking_lot::Mutex;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{atomic::AtomicUsize, Arc},
    time::Duration,
};
use typed_db::{sqlite_impl::SqliteDatabase, Table};
use url::Url;

use crate::{
    avail::Avail,
    backend_api_trait::MockBackendApiTrait,
    endpoints::defns::api::{
        installations::GetInstallationAccessTokenQueryParams, websocket_updates::ServerMsg,
    },
    github_api_trait::MockGithubApiTrait,
    sync_engine::optimistic::db::tests::init_executor,
    types::{
        installation_access_token_row::InstallationAccessToken, issue::Issue,
        repository::Repository, user::User,
    },
    utils::LogErr,
};

use super::{
    optimistic::db::{ReactivityTrackers, SerializedId},
    websocket_updates::transport::tests::MockTransport,
    DbSubscription, SyncEngine,
};

#[allow(dead_code)]
#[derive(Clone, Default, Debug)]
struct NumTimesHit {
    hit: Arc<AtomicUsize>,
}

#[allow(dead_code)]
impl NumTimesHit {
    pub fn increment(&self) {
        self.hit.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn expect_and_reset(&self, n: usize) -> bool {
        self.hit
            .compare_exchange(
                n,
                0,
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
            )
            .is_ok()
    }
}

#[derive(Debug)]
struct AnyError(Box<dyn std::error::Error>);

impl<E: std::error::Error + 'static> From<E> for AnyError {
    fn from(value: E) -> Self {
        Self(Box::new(value))
    }
}

#[tokio::test]
async fn testing_optimistic_create() -> Result<(), AnyError> {
    init_executor();

    // Let's buckle up. Get all our stuff ready.
    let user = User::default();
    let repository = Repository::default();
    let realistic_issue_id = 23423;
    let actual_issue_number = 9098;
    let github_issue = github_api::models::Issue {
        id: realistic_issue_id,
        number: actual_issue_number,
        ..Default::default()
    };
    let installation_id = repository.installation_id;

    // Setup our mock transport and mock backend api.
    let (mock_transport, mut mock_transport_handler) = MockTransport::new();
    let mock_transport = Rc::new(RefCell::new(Some(mock_transport)));
    let mut mock_backend_api = MockBackendApiTrait::new();
    let mut mock_github_api = MockGithubApiTrait::new();

    let title = IssuesCreateRequestTitle::String("fancy title".to_string());

    let create_issues_hit = Arc::new(NumTimesHit::default());
    let create_issues_hit_clone = create_issues_hit.clone();
    let (mut create_issues_resp_sender, create_issues_resp_receiver) =
        mpsc::channel::<github_api::models::Issue>(10);
    let create_issues_resp_receiver = Arc::new(Mutex::new(create_issues_resp_receiver));

    let title_clone = title.clone();
    mock_github_api
        .expect_issues_slash_create()
        .times(..=1)
        .withf(move |_, _, _, issue_create_request_in_mock| {
            issue_create_request_in_mock.title == title_clone
        })
        .returning_st(move |_, _, _, _| {
            let create_issues_resp_receiver = create_issues_resp_receiver.clone();
            let create_issues_hit_clone = create_issues_hit_clone.clone();
            Box::pin(async move {
                let ret = create_issues_resp_receiver
                    .lock()
                    .next()
                    .await
                    .expect("channel closed");
                create_issues_hit_clone.increment();
                Ok(ret)
            })
        });

    let mock_github_api = Arc::new(Mutex::new(mock_github_api));

    mock_backend_api
        .expect_get_installation_access_token()
        .times(..=1)
        .with(predicate::eq(GetInstallationAccessTokenQueryParams {
            installation_id,
        }))
        .returning(|_| Box::pin(async { Ok(InstallationAccessToken::default()) }));

    mock_backend_api
        .expect_get_domain()
        .with()
        .returning(|| Url::parse("https://bcd.efg.xyz").unwrap());

    let mock_backend_api = Arc::new(Mutex::new(mock_backend_api));
    // let mock_backend_api_clone = mock_backend_api.clone();

    let se = SyncEngine::<SqliteDatabase, _, _, _>::new(
        mock_backend_api,
        move |_| {
            let mock_transport = mock_transport.clone();
            async move { Ok(mock_transport.borrow_mut().take().unwrap()) }
        },
        mock_github_api.clone(),
        ":memory:".into(),
    )
    .await?;
    let se_clone = se.clone();

    Executor::spawn_local(async move {
        let _ = se_clone.recv_websocket_updates().await.log_err();
    });

    se.db.put(&user).await?;
    se.db.put(&repository).await?;

    // Subscribe to changes that pertain to the as-of-yet-not-created issue.
    let bulk_subscriber_hit = Arc::new(NumTimesHit::default());
    let bulk_subscriber_hit_clone = bulk_subscriber_hit.clone();
    let bulk_db_subscription = DbSubscription {
        original_reactivity_trackers: ReactivityTrackers {
            stores_read_in_bulk: hashset![Issue::NAME],
            ..Default::default()
        },
        func: Arc::new(move || {
            bulk_subscriber_hit_clone.increment();
        }),
    };

    let _ = se.db_subscriptions.add(bulk_db_subscription);

    let issue_create_request = IssuesCreateRequest {
        title: title.clone(),
        ..Default::default()
    };
    let optimistic_issue_id =
        se.create_issue(&installation_id, &user, &repository, issue_create_request)?;

    assert!(bulk_subscriber_hit.expect_and_reset(1));

    let issues = se.db.get_all_optimistically::<Issue>().await?;
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].id, optimistic_issue_id);
    assert_eq!(issues[0].title, Avail::Yes("fancy title".into()));
    assert!(issues[0].is_optimistic);

    let single_issue = se
        .db
        .get_optimistically::<Issue>(&optimistic_issue_id)
        .await?
        .unwrap();

    assert!(single_issue.is_optimistic);
    assert!(single_issue.id == optimistic_issue_id);
    assert!(single_issue.title == Avail::Yes("fancy title".into()));

    // Now let's send the reply to the create_issues_resp_receiver.
    create_issues_resp_sender.send(github_issue).await?;
    wait_for(&move || create_issues_hit.expect_and_reset(1)).await;

    let single_subscriber_hit = Arc::new(NumTimesHit::default());
    let single_subscriber_hit_clone = single_subscriber_hit.clone();
    let single_db_subscription = DbSubscription {
        original_reactivity_trackers: ReactivityTrackers {
            stores_read_by_id: hashmap![Issue::NAME => hashset![SerializedId::new_from_id::<Issue>(&optimistic_issue_id)]],
            ..Default::default()
        },
        func: Arc::new(move || {
            single_subscriber_hit_clone.increment();
        }),
    };

    let _ = se.db_subscriptions.add(single_db_subscription);

    // Let's send in the webhook.
    let now = Timestamp::now().to_string();
    let webhook_body = WebhookBody::Issues(Issues::Opened {
        changes: None,
        enterprise: None,
        installation: Some(SomethingWithAnId {
            id: *installation_id,
        }),
        issue: IssuesOpenedIssue {
            id: realistic_issue_id,
            title: "fancy title".to_string(),
            updated_at: now.clone(),
            created_at: now,
            ..Default::default()
        },
        organization: Default::default(),
        repository: Default::default(),
        sender: Default::default(),
    });

    mock_transport_handler
        .sender
        .send(ServerMsg {
            body: webhook_body,
            created_at: Timestamp::now(),
        })
        .await?;

    wait_for(&move || bulk_subscriber_hit.expect_and_reset(1)).await;
    wait_for(&move || single_subscriber_hit.expect_and_reset(1)).await;

    let txn = se.db.txn().with_table::<Issue>().build();

    // Make sure that the optimistic thing is removed from bulk reads.
    let issues = txn.table::<Issue>().get_all_optimistically().await?;

    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].id, realistic_issue_id.into());
    assert_eq!(issues[0].title, Avail::Yes("fancy title".into()));
    assert!(!issues[0].is_optimistic);

    // Make sure that fetching by optimistic id returns the realistic thing.
    let issue = txn
        .table::<Issue>()
        .get_optimistically(&optimistic_issue_id)
        .await?;
    assert_eq!(issue, None);

    Ok(())
}

#[cfg(feature = "hydrate")]
#[cfg(not(feature = "ssr"))]
#[track_caller]
async fn wait_for(assertion: &dyn Fn() -> bool) {
    let start = leptos::prelude::window().performance().unwrap().now();
    let mut delay_ms = 20.0;

    while (leptos::prelude::window().performance().unwrap().now() - start) < 2000.0 {
        if assertion() {
            return;
        }
        delay_ms = (delay_ms * 1.5_f64).min(1000.0);
        gloo_timers::future::TimeoutFuture::new(delay_ms as u32).await;
        tick().await;
    }

    // Alert with Location and message.
    panic!("wait_for failed");
}

#[cfg(feature = "ssr")]
#[track_caller]
async fn wait_for(assertion: &(dyn Fn() -> bool + Sync)) {
    use futures::future::FutureExt;

    let timeout = select(
        Box::pin(async {
            let mut delay_ms = 20.0;

            loop {
                if assertion() {
                    return;
                }
                delay_ms = (delay_ms * 1.5_f64).min(1000.0);
                tokio::time::sleep(Duration::from_millis(delay_ms as u64)).await;
                Executor::tick().await;
            }
        }),
        tokio::time::sleep(Duration::from_secs(2)).boxed(),
    );

    match timeout.await {
        Either::Left(_) => {}
        Either::Right(_) => panic!("wait_for failed"),
    }
}
