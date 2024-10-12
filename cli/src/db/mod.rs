mod schema;

use crate::error::Result;
use deadpool_diesel::sqlite::{Manager, Pool};
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel_migrations::MigrationHarness;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use schema::*;
use std::fs;
use std::path::PathBuf;

const CONFIG_PATH: &str = "~/.config/heimisch/";

pub fn get_or_create_config_dir() -> PathBuf {
    let expanded_path = shellexpand::tilde(CONFIG_PATH);
    let path = PathBuf::from(expanded_path.into_owned());

    if let Err(e) = &fs::create_dir_all(&path) {
        panic!("Failed to create config directory: {:?}", e);
    }

    path
}

pub fn establish_pool() -> Pool {
    let database_url = get_or_create_config_dir().join("db.sqlite");
    let database_url = database_url.to_str().expect("can't convert OsStr to str");
    let manager = Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);
    deadpool_diesel::sqlite::Pool::builder(manager)
        .build()
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub async fn get_migrated_pool() -> Result<Pool> {
    let pool = establish_pool();
    pool.get()
        .await?
        .interact(|conn| {
            conn.run_pending_migrations(MIGRATIONS)
                .expect("Couldn't run migrations in sqlite db.");
        })
        .await?;
    Ok(pool)
}

#[allow(unused)]
pub async fn get_user_access_token(pool: &Pool) -> Result<Option<String>> {
    Ok(pool
        .get()
        .await?
        .interact(|conn| {
            access_tokens::table
                .select(access_tokens::access_token)
                .first(conn)
        })
        .await??)
}

pub async fn delete_access_token_if_exists(pool: &Pool) -> Result<()> {
    pool.get()
        .await?
        .interact(move |conn| diesel::delete(access_tokens::table).execute(conn))
        .await??;
    Ok(())
}

pub async fn set_access_token(pool: &Pool, access_token: String) -> Result<()> {
    pool.get()
        .await?
        .interact(move |conn| {
            insert_into(access_tokens::table)
                .values(access_tokens::access_token.eq(&access_token))
                .on_conflict(access_tokens::access_token)
                .do_update()
                .set(access_tokens::access_token.eq(&access_token))
                .execute(conn)
        })
        .await??;
    Ok(())
}
