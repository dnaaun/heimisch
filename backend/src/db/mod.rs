mod schema;

use diesel::{dsl::{delete, exists}, insert_into, prelude::*, select};

use schema::temp_auths;

use crate::error::Result;

trait DbContext {
    fn get_conn(&self) -> &deadpool_diesel::postgres::Object;
}

impl DbContext for &deadpool_diesel::postgres::Object {
    fn get_conn(&self) -> &deadpool_diesel::postgres::Object {
        &self
    }
}

async fn store_csrf_token(db_context: &dyn DbContext, csrf_token: String) -> Result<()> {
    let conn = db_context.get_conn();
    conn.interact(|conn| {
        insert_into(temp_auths::table)
            .values(temp_auths::csrf_token.eq(csrf_token))
            .execute(conn)
    })
    .await??;
    Ok(())
}

async fn does_csrf_token_exist(db_context: &dyn DbContext, csrf_token: String) -> Result<bool> {
    let conn = db_context.get_conn();
    Ok(conn
        .interact(|conn| {
            select(exists(
                temp_auths::table.filter(temp_auths::csrf_token.eq(csrf_token)),
            ))
            .get_result(conn)
        })
        .await??)
}

async fn delete_csrf_tokens_older_than(
    db_context: &dyn DbContext,
    instant: std::time::SystemTime,
) -> Result<()> {
    let conn = db_context.get_conn();
temp_auths::created_at.gt(instant);
    conn
        .interact(move |conn| {
            delete(temp_auths::table.filter(temp_auths::created_at.lt(instant))).execute(conn)
        })
    .await??;
    Ok(())
}
