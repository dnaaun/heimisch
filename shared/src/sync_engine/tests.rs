use any_spawner::Executor;
use futures::SinkExt;
use github_api::models::{IssuesCreateRequest, IssuesCreateRequestTitle};
use github_webhook_body::{Issues, IssuesOpenedIssue, SomethingWithAnId, WebhookBody};
use jiff::Timestamp;
use leptos::task::tick;
use macros::leptos_test_setup;
use maplit::{hashmap, hashset};
use mockall::predicate;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{atomic::AtomicUsize, Arc},
};
use typesafe_idb::Store;
use url::Url;

use crate::{
    avail::Avail,
    backend_api_trait::{BackendApiTrait, MockBackendApiTrait},
    endpoints::defns::api::{
        installations::GetInstallationAccessTokenQueryParams, websocket_updates::ServerMsg,
    },
    github_api_trait::MockGithubApiTrait,
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

#[leptos_test_setup]
async fn testing_optimistic_create() {
    let user = User::default();
    let repository = Repository::default();
    let actual_issue_id = 23423;
    let actual_issue_number = 9098;
    let github_issue = github_api::models::Issue {
        id: actual_issue_id,
        number: actual_issue_number,
        ..Default::default()
    };
    let installation_id = repository.installation_id;
    let (mock_transport, mut mock_transport_handler) = MockTransport::new();
    let mock_transport = Rc::new(RefCell::new(Some(mock_transport)));
    let mut mock_backend_api = MockBackendApiTrait::new();
    let mut mock_github_api = MockGithubApiTrait::new();

    let title = IssuesCreateRequestTitle::String("fancy title".to_string());
    let issue_create_request = IssuesCreateRequest {
        title: title.clone(),
        body: Some("fancy body".to_string()),
        ..Default::default()
    };

    let create_issues_hit = Arc::new(NumTimesHit::default());
    let create_issues_hit_clone = create_issues_hit.clone();
    mock_github_api
        .expect_issues_slash_create()
        .once()
        .withf(move |_, _, _, issue_create_request_in_mock| {
            issue_create_request_in_mock.title == title.clone()
        })
        .returning(move |_, _, _, _| {
            let github_issue = github_issue.clone();
            let create_issues_hit_clone = create_issues_hit_clone.clone();
            Box::pin(async move {
                create_issues_hit_clone.increment();
                Ok(github_issue.clone())
            })
        });

    let mock_github_api = Rc::new(RefCell::new(mock_github_api));

    mock_backend_api
        .expect_get_installation_access_token()
        .once()
        .with(predicate::eq(GetInstallationAccessTokenQueryParams {
            installation_id,
        }))
        .returning(|_| Box::pin(async { Ok(InstallationAccessToken::default()) }));

    mock_backend_api
        .expect_get_domain()
        .with()
        .returning(|| Url::parse("https://bcd.efg.xyz").unwrap());

    let mock_backend_api = Rc::new(RefCell::new(mock_backend_api));
    let mock_backend_api_clone = mock_backend_api.clone();

    let sync_engine = SyncEngine::new(
        mock_backend_api,
        move |_| {
            let mock_transport = mock_transport.clone();
            async move { Ok(mock_transport.borrow_mut().take().unwrap()) }
        },
        mock_github_api.clone(),
    )
    .await
    .unwrap();
    let sync_engine_clone = sync_engine.clone();
    tracing::info!(
        "The get_backend() is {:?}",
        mock_backend_api_clone.borrow().get_domain()
    );
    Executor::spawn_local(async move {
        tracing::info!("In 'coroutine' that will receive websocket updates");
        tracing::info!(
            "The get_backend() is {:?}",
            mock_backend_api_clone.borrow().get_domain()
        );
        let _ = sync_engine_clone.recv_websocket_updates().await.log_err();
    });

    let txn = sync_engine
        .db
        .txn()
        .with_store::<User>()
        .with_store::<Repository>()
        .with_store::<Issue>()
        .read_write()
        .build();
    txn.object_store::<User>()
        .unwrap()
        .put(&user)
        .await
        .unwrap();
    txn.object_store::<Repository>()
        .unwrap()
        .put(&repository)
        .await
        .unwrap();
    txn.commit().unwrap();

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

    let _ = sync_engine.db_subscriptions.add(bulk_db_subscription);

    let optimistic_issue_id = sync_engine
        .create_issue(&installation_id, &user, &repository, issue_create_request)
        .unwrap();

    assert!(bulk_subscriber_hit.expect_and_reset(1));

    let txn = sync_engine.db.txn().with_store::<Issue>().build();
    let issues = txn
        .object_store::<Issue>()
        .unwrap()
        .get_all_optimistically()
        .await
        .unwrap();
    drop(txn);
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].id, optimistic_issue_id);
    assert_eq!(issues[0].title, Avail::Yes("fancy title".into()));
    assert!(issues[0].is_optimistic);

    wait_for(move || create_issues_hit.expect_and_reset(1)).await;
    assert!(bulk_subscriber_hit.expect_and_reset(0));

    let txn = sync_engine.db.txn().with_store::<Issue>().build();
    let single_issue = txn
        .object_store::<Issue>()
        .unwrap()
        .get_optimistically(&optimistic_issue_id)
        .await
        .unwrap()
        .unwrap();
    drop(txn);
    assert!(single_issue.is_optimistic);
    assert!(single_issue.id == optimistic_issue_id);
    assert!(single_issue.title == Avail::Yes("fancy title".into()));

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

    let _ = sync_engine.db_subscriptions.add(single_db_subscription);

    // Let's send in the webhook.
    let now = Timestamp::now().to_string();
    let webhook_body = WebhookBody::Issues(Issues::Opened {
        changes: None,
        enterprise: None,
        installation: Some(SomethingWithAnId {
            id: *installation_id,
        }),
        issue: IssuesOpenedIssue {
            id: actual_issue_id,
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
        .await
        .unwrap();

    wait_for(move || bulk_subscriber_hit.expect_and_reset(1)).await;
    wait_for(move || single_subscriber_hit.expect_and_reset(1)).await;
}

#[allow(dead_code)] // Not sure why this is necessary since wait_for is indeed used.
#[track_caller]
async fn wait_for<F>(assertion: F)
where
    F: Fn() -> bool,
{
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
