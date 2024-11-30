use crate::error::Result;
use diesel::{
    dsl::{exists, select},
    prelude::*,
};
use std::time::SystemTime;

use super::schema::temp_auths::{self, *};
use deadpool_diesel::postgres::Pool;
use diesel::{dsl::delete, insert_into};

pub async fn store_csrf_token(pool: impl AsRef<Pool>, csrf_token_arg: String) -> Result<()> {
    let conn = pool.as_ref().get().await?;
    conn.interact(|conn| {
        insert_into(table)
            .values((
                csrf_token.eq(csrf_token_arg),
                created_at.eq(SystemTime::now()),
            ))
            .execute(conn)
    })
    .await??;
    Ok(())
}

pub async fn does_csrf_token_exist(pool: impl AsRef<Pool>, csrf_token_arg: String) -> Result<bool> {
    let conn = pool.as_ref().get().await?;
    Ok(conn
        .interact(|conn| -> std::result::Result<bool, diesel::result::Error> {
            select(exists(table.filter(csrf_token.eq(csrf_token_arg)))).get_result(conn)
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

pub async fn delete_csrf_token(pool: impl AsRef<Pool>, csrf_token_arg: String) -> Result<()> {
    let conn = pool.as_ref().get().await?;
    conn.interact(move |conn| delete(table.filter(csrf_token.eq(csrf_token_arg))).execute(conn))
        .await??;
    Ok(())
}
