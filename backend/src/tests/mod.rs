mod parse_request;

use std::{cell::LazyCell, future::Future, path::PathBuf, time::SystemTime};

use crate::error::Error as CrateError;
use assert_json_diff::assert_json_include;
use axum_test::TestServer;
use backtrace::Backtrace;
use deadpool_diesel::postgres::Pool;
use diesel::{QueryDsl, RunQueryDsl};
use diesel_test::{
    postgres::{ParsingDbUrlError, PostgresDbUrlFactory},
    DieselTestConfig,
};
use parking_lot::Mutex;
use parse_request::ParsedHttpRequest;
use serde_json::Value;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::{
    config::init_config,
    db::{self, insert_installation_if_not_exists, upsert_login_user},
    get_router, MIGRATIONS,
};

const DB_COUNTER: LazyCell<Mutex<u32>> = LazyCell::new(|| Default::default());

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

async fn with_test_server<Fut: Future>(
    func: impl FnOnce(Pool, TestServer) -> Fut,
) -> TestResult<Fut::Output> {
    // setup tracing
    let filter = EnvFilter::new("INFO");
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    // TODO: Improve test setup so that environment variables are not required.
    let config = init_config().await;
    let db_url_factory =
        PostgresDbUrlFactory::new(&config.db.url, None, DB_COUNTER, vec!["uuid-ossp"])
            .map_err(TestError::new_parsing_db_url)?;
    let mut diesel_test_config = DieselTestConfig {
        migrations: MIGRATIONS,
        db_url_factory,
    };

    Ok(diesel_test_config
        .with_pool(|pool, db_url| async {
            let mut config = config.clone();
            config.db.url = db_url;

            let test_server = TestServer::new(get_router(config, None).await)
                .map_err(TestError::new_test_server)?;

            Ok::<Fut::Output, TestError>(func(pool, test_server).await)
        })
        .await?)
}

#[tokio::test]
async fn test_simple_webhook_delivery() -> TestResult<()> {
    with_test_server(|pool, server| async move {
        let pool = Box::new(pool); // makes it easier to pass it to impl AsRef<Pool> args
        let login_user: db::UpsertLoginUser = Default::default();
        let expected_installation_id = 56385187; // Must match the installation id in the fixture.
        let installation = db::Installation {
            id: expected_installation_id,
            created_at: SystemTime::now(),
            github_user_id: login_user.github_user_id,
        };
        upsert_login_user(&pool, login_user).await?;
        insert_installation_if_not_exists(&pool, installation).await?;
        let req = ParsedHttpRequest::from_file(&PathBuf::from(
            "./test_fixtures/issue_comment_webhook.request",
        ))
        .await?;
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

        let resp = req.make(&server).await;

        use db::schema::webhooks::*;
        let conn = pool.get().await.map_err(CrateError::from)?;
        let (actual_webhook_content, actual_installation_id, actual_id): (Value, i64, i64) = conn
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

        println!("{}", resp.text());
        resp.assert_status_ok();
        Ok::<_, TestError>(())
    })
    .await?
}
