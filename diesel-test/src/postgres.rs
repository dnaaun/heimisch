use diesel_migrations::EmbeddedMigrations;
use itertools::Itertools;
use std::{cell::LazyCell, str::FromStr, sync::Arc};
use tokio_postgres::Client;

use diesel::PgConnection;
use parking_lot::Mutex;
use pg_connection_string::ConnectionString;

use crate::{DbUrlFactory, DieselTestConfig};

pub struct PostgresDbUrlFactory {
    conn_str_parsed: ConnectionString,
    db_name_prefix: String,
    db_counter: LazyCell<Arc<Mutex<u32>>>,
    extension_names: Vec<&'static str>,
}

pub type ParsingDbUrlError = <ConnectionString as FromStr>::Err;

#[derive(Debug)]
pub enum Error {
    TokioPg(tokio_postgres::Error),
    DieselConn(diesel::ConnectionError),
    Query(diesel::result::Error),
    ParsingDbUrl(ParsingDbUrlError),
}

impl From<tokio_postgres::Error> for Error {
    fn from(value: tokio_postgres::Error) -> Self {
        Self::TokioPg(value)
    }
}

impl From<diesel::ConnectionError> for Error {
    fn from(value: diesel::ConnectionError) -> Self {
        Self::DieselConn(value)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        Self::Query(value)
    }
}

impl PostgresDbUrlFactory {
    /// `db_counter` is a lazy cell to encourage/force you to have a global counter.
    pub fn new(
        base_db_url: &str,
        db_name_prefix: Option<String>,
        db_counter: LazyCell<Arc<Mutex<u32>>>,
        extension_names: Vec<&'static str>,
    ) -> Result<Self, ParsingDbUrlError> {
        let conn_str_parsed = ConnectionString::from_str(base_db_url)?;
        let db_name_prefix = db_name_prefix
            .or(conn_str_parsed.database.clone())
            .unwrap_or("diesel_test".to_owned());
        Ok(Self {
            conn_str_parsed,
            db_name_prefix,
            db_counter,
            extension_names,
        })
    }
}

async fn get_tokio_pg_conn(db_url: &str) -> Result<tokio_postgres::Client, Error> {
    let (client, connection) = tokio_postgres::connect(db_url, tokio_postgres::NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

/// We're forced to use tokio-postgres here because I couldn't find a way to have no transactions
/// in diesel, which is necessary to run DROP DATABASE.
#[async_trait::async_trait]
impl DbUrlFactory for PostgresDbUrlFactory {
    type Conn = PgConnection;
    type Err = Error;

    async fn prep_next_db_url(&mut self) -> Result<String, Self::Err> {
        let mut conn_str_parsed = self.conn_str_parsed.clone();
        let cur_count = {
            let mut counter = self.db_counter.lock();
            *counter += 1;
            *counter
        };

        conn_str_parsed.database = None;
        let conn_str_no_db_url = conn_str_parsed.to_string();

        let client = get_tokio_pg_conn(&conn_str_no_db_url).await?;
        let db_name = format!("{}_{}", self.db_name_prefix, cur_count);
        tear_down_logic(&client, &db_name).await?;
        client
            .execute(&format!("CREATE DATABASE {db_name};\n"), &[])
            .await?;
        drop(client);

        conn_str_parsed.database = Some(db_name.clone());
        let conn_str = conn_str_parsed.to_string();
        let client = get_tokio_pg_conn(&conn_str).await?;
        let create_ext_cmds = self
            .extension_names
            .iter()
            .map(|n| format!("CREATE EXTENSION IF NOT EXISTS \"{n}\";"))
            .join("\n");
        client.batch_execute(&create_ext_cmds).await?;

        Ok(conn_str)
    }

    /// Will assume db_url is a valid Postgres address, and has the database name
    /// in it. It will panic otherwise.
    async fn tear_down_db_url(&mut self, _db_url: &str) -> Result<(), Error> {
        // let conn_str_parsed = ConnectionString::from_str(db_url)
        //     .expect("db_url expected to be valid postgrse address string");
        // let db_name = conn_str_parsed
        //     .database
        //     .expect("db_url expected to contain database name");
        // let client = get_tokio_pg_conn(&db_url).await?;
        // tear_down_logic(&client, &db_name).await
        Ok(())
    }
}

async fn tear_down_logic(client: &Client, db_name: &str) -> Result<(), Error> {
    client
        .batch_execute(&format!(
            "
        DROP DATABASE IF EXISTS {db_name} WITH (FORCE);"
        ))
        .await?;
    Ok(())
}

impl<D: DbUrlFactory> DieselTestConfig<D> {
    pub fn for_pg(
        base_db_url: &str,
        migrations: EmbeddedMigrations,
        db_counter: LazyCell<Arc<Mutex<u32>>>,
        db_name_prefix: Option<String>,
        extension_names: Vec<&'static str>,
    ) -> Result<DieselTestConfig<PostgresDbUrlFactory>, ParsingDbUrlError> {
        let db_url_factory =
            PostgresDbUrlFactory::new(base_db_url, db_name_prefix, db_counter, extension_names)?;
        Ok(DieselTestConfig {
            migrations,
            db_url_factory,
        })
    }
}
