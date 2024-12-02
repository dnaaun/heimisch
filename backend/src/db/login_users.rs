use std::time::SystemTime;

use crate::error::Result;
use diesel::prelude::*;
use shared::{endpoints::defns::api::auth::finish::GithubAccessToken, types::user::UserId};

use super::schema::login_users::{self, *};
use deadpool_diesel::postgres::Pool;

#[derive(Insertable, Clone, Debug, Default)]
#[diesel(table_name = login_users)]
pub struct UpsertLoginUser {
    pub github_user_id: UserId,
    pub github_username: String,
    pub github_email: Option<String>,
    pub github_access_token: GithubAccessToken,
}

#[derive(Queryable, Clone, Debug, Default)]
#[diesel(table_name = login_users)]
pub struct LoginUser {
    pub github_user_id: UserId,
    pub github_username: String,
    pub github_email: Option<String>,
    pub github_access_token: GithubAccessToken,
    pub last_last_in_touch_at: Option<SystemTime>,
}

pub async fn upsert_login_user(pool: impl AsRef<Pool>, user: UpsertLoginUser) -> Result<()> {
    let conn = pool.as_ref().get().await?;
    conn.interact(move |conn| {
        diesel::insert_into(table)
            .values(&user)
            .on_conflict(github_user_id)
            .do_update()
            .set((
                github_username.eq(&user.github_username),
                github_email.eq(&user.github_email),
                github_access_token.eq(&user.github_access_token),
            ))
            .execute(conn)
    })
    .await??;

    Ok(())
}

pub async fn get_login_user(pool: impl AsRef<Pool>, id_arg: &UserId) -> Result<Option<LoginUser>> {
    let conn = pool.as_ref().get().await?;
    let id_arg = *id_arg;
    Ok(conn
        .interact(move |conn| {
            table
                .filter(github_user_id.eq(id_arg))
                .select(all_columns)
                .first::<LoginUser>(conn)
                .optional()
        })
        .await??)
}
