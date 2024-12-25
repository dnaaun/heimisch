use std::{ops::Deref, time::SystemTime};

use super::schema::installations::{self, *};
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

use shared::types::{installation::InstallationId, user::UserId};

use crate::error::{Error, Result};

#[derive(Insertable, Queryable, Clone)]
#[diesel(table_name = installations)]
pub struct Installation {
    pub id: InstallationId,
    pub created_at: SystemTime,
    pub github_user_id: UserId,
}
pub async fn insert_installation_if_not_exists(
    pool: impl AsRef<Pool>,
    installation_arg: Installation,
) -> Result<()> {
    let conn = pool.as_ref().get().await?;
    conn.interact(move |conn| {
        diesel::insert_into(table)
            .values(&installation_arg)
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

pub async fn get_installations(
    pool: impl AsRef<Pool>,
    user_id_arg: UserId,
) -> Result<Vec<Installation>> {
    let conn = pool.as_ref().get().await?;
    Ok(conn
        .interact(move |conn| {
            Ok::<_, Error>(table
                .select(all_columns)
                .filter(github_user_id.eq(user_id_arg))
                .load(conn)?)
        })
        .await??)
}
