pub mod schema;

use std::{ops::Deref, time::SystemTime};

use deadpool_diesel::postgres::Pool;
use diesel::{
    dsl::{delete, exists},
    insert_into,
    prelude::*,
    select,
};

use github_webhook_body::WebhookBody;
use schema::temp_auths;
use shared::types::installation::InstallationId;

use crate::error::Result;

pub async fn store_csrf_token(pool: impl AsRef<Pool>, csrf_token: String) -> Result<()> {
    let conn = pool.as_ref().get().await?;
    conn.interact(|conn| {
        insert_into(temp_auths::table)
            .values((
                temp_auths::csrf_token.eq(csrf_token),
                temp_auths::created_at.eq(SystemTime::now()),
            ))
            .execute(conn)
    })
    .await??;
    Ok(())
}

pub async fn does_csrf_token_exist(pool: impl AsRef<Pool>, csrf_token: String) -> Result<bool> {
    let conn = pool.as_ref().get().await?;
    Ok(conn
        .interact(|conn| {
            select(exists(
                temp_auths::table.filter(temp_auths::csrf_token.eq(csrf_token)),
            ))
            .get_result(conn)
        })
        .await??)
}

#[allow(unused)]
pub async fn delete_csrf_tokens_older_than(
    pool: impl AsRef<Pool>,
    instant: SystemTime,
) -> Result<()> {
    let conn = pool.as_ref().get().await?;
    temp_auths::created_at.gt(instant);
    conn.interact(move |conn| {
        delete(temp_auths::table.filter(temp_auths::created_at.lt(instant))).execute(conn)
    })
    .await??;
    Ok(())
}

pub async fn delete_csrf_token(pool: impl AsRef<Pool>, csrf_token: String) -> Result<()> {
    let conn = pool.as_ref().get().await?;
    conn.interact(move |conn| {
        delete(temp_auths::table.filter(temp_auths::csrf_token.eq(csrf_token))).execute(conn)
    })
    .await??;
    Ok(())
}

#[derive(Insertable, Clone, Default)]
#[diesel(table_name = schema::login_users)]
pub struct LoginUser {
    pub github_user_id: i64,
    pub github_username: String,
    pub github_email: Option<String>,
    pub github_access_token: String,
}

pub async fn upsert_user(pool: impl AsRef<Pool>, user: LoginUser) -> Result<()> {
    let conn = pool.as_ref().get().await?;
    conn.interact(move |conn| {
        diesel::insert_into(schema::login_users::table)
            .values(&user)
            .on_conflict(schema::login_users::github_user_id)
            .do_update()
            .set((
                schema::login_users::github_username.eq(&user.github_username),
                schema::login_users::github_email.eq(&user.github_email),
                schema::login_users::github_access_token.eq(&user.github_access_token),
            ))
            .execute(conn)
    })
    .await??;

    Ok(())
}

#[derive(Insertable, Queryable, Clone)]
#[diesel(table_name = schema::installations)]
pub struct Installation {
    pub id: i64,
    pub created_at: SystemTime,
    pub github_user_id: i64,
}
pub async fn insert_installation_if_not_exists(
    pool: impl AsRef<Pool>,
    installation: Installation,
) -> Result<()> {
    use schema::installations::*;
    let conn = pool.as_ref().get().await?;
    conn.interact(move |conn| {
        diesel::insert_into(table)
            .values(&installation)
            .on_conflict(id)
            .do_nothing()
            .execute(conn)
    })
    .await??;

    Ok(())
}

pub async fn get_installation(
    pool: impl AsRef<Pool>,
    id_arg: InstallationId,
) -> Result<Option<Installation>> {
    let conn = pool.as_ref().get().await?;
    use schema::installations::*;
    Ok(conn
        .interact(move |conn| -> QueryResult<Option<Installation>> {
            Ok(table
                .select(all_columns)
                .filter(id.eq(id_arg.deref()))
                .limit(1)
                .load(conn)?
                .into_iter()
                .next())
        })
        .await??)
}

pub async fn upsert_webhook(
    pool: impl AsRef<Pool>,
    id_arg: i64,
    installation_id_arg: InstallationId,
    webhook_content_arg: WebhookBody,
) -> Result<()> {
    let webhook_content_arg = serde_json::to_value(webhook_content_arg).expect("");
    let conn = pool.as_ref().get().await?;
    use schema::webhooks::*;
    conn.interact(move |conn| {
        diesel::insert_into(table)
            .values((
                id.eq(id_arg),
                installation_id.eq(installation_id_arg.deref()),
                webhook_content.eq(&webhook_content_arg),
                created_at.eq(SystemTime::now()),
            ))
            .on_conflict(id)
            .do_update()
            .set((
                installation_id.eq(installation_id_arg.deref()),
                webhook_content.eq(&webhook_content_arg),
            ))
            .execute(conn)
    })
    .await??;

    Ok(())
}
