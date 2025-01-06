use crate::{
    config::Config,
    custom_github_api::{get_user_access_token_request, ATResp},
    db::get_login_user,
    utils::gen_rand_string,
};
pub mod endpoint_test_client;
mod parse_request;

use std::{
    cell::LazyCell,
    collections::HashMap,
    future::Future,
    path::PathBuf,
    sync::Arc,
    time::{Duration, SystemTime},
};

use crate::error::Error as CrateError;
use assert_json_diff::assert_json_include;
use axum_test::{TestResponse, TestServer, WsMessage};
use backtrace::Backtrace;
use deadpool_diesel::postgres::Pool;
use derive_more::derive::AsRef;
use diesel::{QueryDsl, RunQueryDsl};
use diesel_test::{
    postgres::{ParsingDbUrlError, PostgresDbUrlFactory},
    DieselTestConfig,
};
use endpoint_test_client::PostEndpointTestClient;
use github_api::{
    apis::users_api::users_slash_get_authenticated_request,
    models::{PrivateUser, UsersGetAuthenticated200Response},
};
use github_webhook_body::WebhookBody;
use http::StatusCode;
use parking_lot::Mutex;
use parse_request::ParsedHttpRequest;
use serde_json::Value;
use shared::{
    endpoints::{
        defns::api::{
            auth::{
                finish::{AuthFinishEndpoint, AuthFinishPayload, GithubAccessToken},
                initiate::AuthInitiateEndpoint,
            },
            websocket_updates::{ServerMsg, WEBSOCKET_UPDATES_ENDPOINT},
        },
        endpoint_client::MaybePageRedirect,
    },
    types::{installation::InstallationId, user::UserId},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use url::Url;
use wiremock::{MockServer, ResponseTemplate};

use crate::{
    config::init_config,
    db::{self, insert_installation_if_not_exists, upsert_login_user},
    get_router, MIGRATIONS,
};

/// Not sure if global mutable state is best practice lol.
#[allow(clippy::declare_interior_mutable_const)]
const DB_COUNTER: LazyCell<Arc<Mutex<u32>>> = LazyCell::new(Default::default);

#[allow(dead_code)]
#[derive(Debug)]
enum TestErrorSource {
    ParsingDbUrl(ParsingDbUrlError),
    TestServer(anyhow::Error),
    App(CrateError),
    ParseRequestFile(Box<dyn std::error::Error>),
    ParseGithubHookId,
}

#[allow(dead_code)]
#[derive(Debug)]
struct TestError {
    source: TestErrorSource,
    backtrace: Backtrace,
}

type TestResult<T, E = TestError> = Result<T, E>;

impl TestError {
    fn new_parsing_db_url(err: ParsingDbUrlError) -> Self {
        Self {
            source: TestErrorSource::ParsingDbUrl(err),
            backtrace: Backtrace::new(),
        }
    }

    fn new_test_server(err: anyhow::Error) -> Self {
        Self {
            source: TestErrorSource::TestServer(err),
            backtrace: Backtrace::new(),
        }
    }
}

impl From<crate::error::Error> for TestError {
    fn from(value: crate::error::Error) -> Self {
        TestError {
            source: TestErrorSource::App(value),
            backtrace: Backtrace::new(),
        }
    }
}

impl From<TestErrorSource> for TestError {
    fn from(value: TestErrorSource) -> Self {
        TestError {
            source: value,
            backtrace: Backtrace::new(),
        }
    }
}

#[derive(AsRef)]
struct TestSetup {
    #[as_ref]
    pool: Pool,
    #[as_ref]
    server: TestServer,
    #[as_ref]
    config: Config,
    github_api_mock_server: MockServer,
    github_non_api_mock_server: MockServer,
}

async fn with_test_server<Fut: Future>(
    func: impl FnOnce(TestSetup) -> Fut,
) -> TestResult<Fut::Output> {
    // setup tracing
    let filter = EnvFilter::new("INFO");
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    // TODO: Improve test setup so that environment variables are not required.
    let mut config = init_config().await;

    let github_api_mock_server = MockServer::start().await;
    let github_non_api_mock_server = MockServer::start().await;
    config.github_api.api_root = github_api_mock_server.uri().parse().unwrap();
    config.github_api.non_api_root = github_non_api_mock_server.uri().parse().unwrap();

    let db_url_factory =
        PostgresDbUrlFactory::new(&config.db.url, None, DB_COUNTER, vec!["uuid-ossp"])
            .map_err(TestError::new_parsing_db_url)?;
    let mut diesel_test_config = DieselTestConfig {
        migrations: MIGRATIONS,
        db_url_factory,
    };

    diesel_test_config
        .with_pool(|pool, db_url| async {
            let mut config = config.clone();
            config.db.url = db_url;

            let server = TestServer::builder()
                .http_transport() // For websocket testing, this is necessary.
                .build(get_router(config.clone()).await)
                .map_err(TestError::new_test_server)?;

            let test_setup = TestSetup {
                pool,
                server,
                config,
                github_api_mock_server,
                github_non_api_mock_server,
            };

            Ok::<Fut::Output, TestError>(func(test_setup).await)
        })
        .await
}

async fn with_user<Fut: Future>(
    func: impl FnOnce(TestSetup, db::UpsertLoginUser) -> Fut,
) -> TestResult<Fut::Output> {
    with_test_server(|test_setup| async {
        let login_user: db::UpsertLoginUser = Default::default();
        upsert_login_user(Box::new(test_setup.pool.clone()), login_user.clone()).await?;
        Ok::<Fut::Output, TestError>(func(test_setup, login_user).await)
    })
    .await?
}

async fn with_logged_in_user<Fut: Future>(
    func: impl FnOnce(TestSetup, db::LoginUser) -> Fut,
) -> TestResult<Fut::Output> {
    with_test_server(|test_setup| async {
        let TestSetup {
            pool: _,
            server,
            config,
            github_api_mock_server,
            github_non_api_mock_server,
        } = &test_setup;
        let initiate_response = server.get(AuthInitiateEndpoint::PATH).save_cookies().await;

        let redirect_url: Url = initiate_response
            .headers()
            .get(http::header::LOCATION)
            .expect("")
            .to_str()
            .expect("")
            .parse()
            .unwrap();

        let query_params = redirect_url
            .query_pairs()
            .map(|(k, v)| (k.to_lowercase(), v.into_owned()))
            .collect::<HashMap<_, _>>();

        let state = query_params.get("state").unwrap().clone();
        let code = gen_rand_string(10);
        let access_token = GithubAccessToken::from(gen_rand_string(10));

        get_user_access_token_request(
            &config.github_api.non_api_root,
            &code,
            &config.github_api.client_id,
            &config.github_api.client_secret,
        )
        .respond_with(ResponseTemplate::new(200).set_body_json(ATResp {
            access_token: access_token.clone(),
        }))
        .unwrap()
        .mount(github_non_api_mock_server)
        .await;

        let user_id = UserId::from(rand::random::<i64>());
        let private_user = PrivateUser {
            id: *user_id.as_ref(),
            ..Default::default()
        };
        users_slash_get_authenticated_request(
            &config.get_gh_api_conf_with_access_token(Some(access_token.as_str().to_owned())),
        )
        .respond_with(ResponseTemplate::new(200).set_body_json(
            UsersGetAuthenticated200Response::Private(Box::new(private_user)),
        ))
        .unwrap()
        .mount(github_api_mock_server)
        .await;

        let _finish_response =
            AuthFinishEndpoint::make_test_request(server, &AuthFinishPayload { state, code }, ())
                .await;

        let user = get_login_user(&test_setup, &user_id)
            .await
            .expect("Running db query in test db failed")
            .expect("Test setup failed. User not found in db.");
        
        func(test_setup, user).await
    })
    .await
}

async fn deliver_issue_comment_webhook_fixture(
    test_setup: &TestSetup,
    github_user_id: UserId,
) -> TestResult<(InstallationId, ParsedHttpRequest, TestResponse)> {
    let expected_installation_id = InstallationId::from(56385187); // Must match the installation id in the fixture.
    let installation = db::Installation {
        id: expected_installation_id,
        created_at: SystemTime::now(),
        github_user_id,
    };
    insert_installation_if_not_exists(test_setup, installation).await?;
    let req = ParsedHttpRequest::from_file(&PathBuf::from(
        "./test_fixtures/issue_comment_webhook.request",
    ))
    .await?;
    let resp = req.clone().make(test_setup.as_ref()).await;

    if resp.status_code() != StatusCode::OK {
        panic!(
            "Webhook delivery had status code {} and text {}",
            resp.status_code(),
            resp.text()
        )
    }

    Ok((expected_installation_id, req, resp))
}

#[tokio::test]
async fn test_simple_webhook_delivery() -> TestResult<()> {
    with_user(|test_setup, login_user| async move {
        let (expected_installation_id, req, resp) =
            deliver_issue_comment_webhook_fixture(&test_setup, login_user.github_user_id).await?;
        let expected_webhook_content = Value::Object(
            [(
                req.headers.get("x-github-event").expect("").to_owned(),
                serde_json::from_slice(&req.body).expect(""),
            )]
            .into_iter()
            .collect(),
        );
        let expected_webhook_id: i64 = req
            .headers
            .get("x-github-hook-id")
            .expect("")
            .parse()
            .map_err(|_| TestErrorSource::ParseGithubHookId)?;

        use db::schema::webhooks::*;
        let conn = test_setup.pool.get().await.map_err(CrateError::from)?;
        let (actual_webhook_content, actual_installation_id, actual_id): (
            Value,
            InstallationId,
            i64,
        ) = conn
            .interact(|conn| {
                table
                    .select((webhook_content, installation_id, id))
                    .first(conn)
            })
            .await
            .map_err(CrateError::from)?
            .map_err(CrateError::from)?;

        assert_json_include!(actual: expected_webhook_content, expected: actual_webhook_content);
        assert_eq!(expected_installation_id, actual_installation_id);
        assert_eq!(expected_webhook_id, actual_id);

        resp.assert_status_ok();
        Ok::<_, TestError>(())
    })
    .await?
}

#[tokio::test]
async fn test_websocket_updates() -> TestResult<()> {
    with_logged_in_user(|test_setup, user| async move {
        let mut ws_request = test_setup
            .server
            .get_websocket(WEBSOCKET_UPDATES_ENDPOINT)
            .save_cookies()
            .await
            .into_websocket()
            .await;

        ws_request.send_message(WsMessage::Ping(vec![])).await;
        match ws_request.receive_message().await {
            WsMessage::Pong(_) => (),
            a => panic!("Unexpecteed message: {a:?}"),
        };

        tokio::time::sleep(Duration::from_secs(2)).await;

        let (_, parsed_webhook_request, _) =
            deliver_issue_comment_webhook_fixture(&test_setup, user.github_user_id).await?;

        let server_msg = tokio::time::timeout(
            Duration::from_secs(2),
            ws_request.receive_json::<ServerMsg>(),
        )
        .await
        .expect("Expected too long to receive a message on the websocket.");

        let expected_webhook_body =
            serde_json::from_slice::<WebhookBody>(&parsed_webhook_request.body).expect("");
        assert_eq!(
            serde_json::to_value(server_msg.body).unwrap(),
            serde_json::to_value(expected_webhook_body).unwrap()
        );

        ws_request.close().await;
        Ok(())
    })
    .await?
}
