pub mod postgres;

use std::fmt::Debug;
use std::future::Future;
use std::panic::{resume_unwind, AssertUnwindSafe};

use deadpool::managed::Object;
use deadpool_diesel::Manager;
use diesel::Connection;
use diesel_migrations::EmbeddedMigrations;
use futures::FutureExt;

pub struct DieselTestConfig<D: DbUrlFactory> {
    pub migrations: EmbeddedMigrations,
    pub db_url_factory: D,
}

/// Something that provides db urls that specify different databases.
#[async_trait::async_trait]
pub trait DbUrlFactory {
    ///  Statically link some implementation of `DbUrlFactory` with the database type it supports.
    type Conn;
    type Err: Debug;
    async fn prep_next_db_url(&mut self) -> Result<String, Self::Err>;
    async fn tear_down_db_url(&mut self, db_url: &str) -> Result<(), Self::Err>;
}

type Pool<D> = deadpool::managed::Pool<
    deadpool_diesel::Manager<<D as DbUrlFactory>::Conn>,
    Object<deadpool_diesel::Manager<<D as DbUrlFactory>::Conn>>,
>;

impl<D> DieselTestConfig<D>
where
    D: DbUrlFactory,
    D::Conn: Connection,
    deadpool_diesel::Manager<D::Conn>: deadpool::managed::Manager,
{
    pub async fn with_pool<Fut: Future>(
        &mut self,
        func: impl FnOnce(Pool<D>, String) -> Fut,
    ) -> Fut::Output {
        let db_url = self
            .db_url_factory
            .prep_next_db_url()
            .await
            .expect("Preparing database failed.");
        let manager = Manager::new(&db_url, deadpool_diesel::Runtime::Tokio1);
        let pool = deadpool::managed::Pool::builder(manager).build().unwrap();

        let result = AssertUnwindSafe(func(pool, db_url.clone()))
            .catch_unwind()
            .await;
        self.db_url_factory
            .tear_down_db_url(&db_url)
            .await
            .unwrap_or_else(|_| {
                panic!("Tearing database down failed. The database url was: {db_url}")
            });

        match result {
            Ok(output) => output,
            Err(err) => resume_unwind(err),
        }
    }
}
