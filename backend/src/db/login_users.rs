use crate::error::Result;
use diesel::prelude::*;

use super::schema::login_users::{self, *};
use deadpool_diesel::postgres::Pool;

#[derive(Insertable, Clone, Default)]
#[diesel(table_name = login_users)]
pub struct LoginUser {
    pub github_user_id: i64,
    pub github_username: String,
    pub github_email: Option<String>,
    pub github_access_token: String,
}

pub async fn upsert_user(pool: impl AsRef<Pool>, user: LoginUser) -> Result<()> {
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
