use crate::error::{DbIntegrityError, Result};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use super::schema::sessions::{self, *};
use deadpool_diesel::postgres::Pool;
use diesel::{dsl::delete, insert_into, prelude::*};
use serde_json::Value;
use tower_sessions::{
    cookie::time::{OffsetDateTime, UtcOffset},
    session::{Id, Record},
};
use uuid::Uuid;

/// A version of `tower_sessions::session::Record` that plays nice with diesel.
#[derive(Insertable, Queryable, Clone, AsChangeset)]
#[diesel(table_name = sessions)]
struct RecordRow {
    pub id: Uuid,
    pub data: Value,
    pub expiry_date: SystemTime,
}

/// GPT-4
fn system_time_from_offset_date_time(offset_date_time: &OffsetDateTime) -> SystemTime {
    let date_time_utc = offset_date_time.to_offset(UtcOffset::UTC);

    let unix_timestamp = date_time_utc.unix_timestamp();

    let nanos = date_time_utc.nanosecond();

    UNIX_EPOCH + Duration::new(unix_timestamp as u64, nanos)
}

/// GPT-4
fn offset_date_time_from_system_time(system_time: SystemTime) -> OffsetDateTime {
    // Calculate the duration from the UNIX epoch
    match system_time.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            // Extract seconds and nanoseconds
            let nanos = u128::from(duration.as_nanos());
            OffsetDateTime::from_unix_timestamp_nanos(
                nanos.try_into().expect("Bruv, if OffsetDateTime::from_unix_timestamp_nanos expects i128,
                I presume that the fact that we got something that a u128 can represent but i128 can't means
                something is really wrong")
            )
                .expect("Timestamp should be within valid range") // Handle potential range error
        }
        Err(_) => panic!("SystemTime is earlier than UNIX_EPOCH"), // Handle system time before epoch case
    }
}

struct RecordDataIsNotMap;

impl TryInto<Record> for RecordRow {
    type Error = RecordDataIsNotMap;

    fn try_into(self) -> std::result::Result<tower_sessions::session::Record, Self::Error> {
        let data_value = match self.data {
            Value::Object(map) => map.into_iter().collect(),
            _ => return Err(RecordDataIsNotMap),
        };
        Ok(Record {
            id: Id(uuid_to_i128(self.id)),
            data: data_value,
            expiry_date: offset_date_time_from_system_time(self.expiry_date),
        })
    }
}

impl From<&Record> for RecordRow {
    fn from(value: &Record) -> Self {
        Self {
            id: i128_to_uuid(value.id.0),
            data: Value::Object(value.data.clone().into_iter().collect()),
            expiry_date: system_time_from_offset_date_time(&value.expiry_date),
        }
    }
}

pub fn i128_to_uuid(i: i128) -> Uuid {
    Uuid::from_u128(i as u128)
}

pub fn uuid_to_i128(u: Uuid) -> i128 {
    u.as_u128() as i128
}

pub async fn get_session(pool: impl AsRef<Pool>, id_arg: &Id) -> Result<Option<Record>> {
    let conn = pool.as_ref().get().await?;
    let id_arg = i128_to_uuid(id_arg.0);
    let row = conn
        .interact(move |conn| {
            table
                .select(all_columns)
                .filter(id.eq(id_arg))
                .first::<RecordRow>(conn)
                .optional()
        })
        .await??;

    Ok(match row {
        Some(row) => {
            let err = DbIntegrityError::SessionsDataIsNotMap {
                session_id: row.id.clone(),
                session_data: row.data.clone(),
            };
            Some(row.try_into().map_err(|_| err)?)
        }
        None => None,
    })
}

pub async fn create_session(pool: impl AsRef<Pool>, record: &Record) -> Result<()> {
    let row: RecordRow = record.into();
    let conn = pool.as_ref().get().await?;
    conn.interact(|conn| insert_into(table).values(row).execute(conn))
        .await??;
    Ok(())
}

pub async fn upsert_session(pool: impl AsRef<Pool>, record: &Record) -> Result<()> {
    let row: RecordRow = record.into();
    let conn = pool.as_ref().get().await?;
    conn.interact(move |conn| {
        insert_into(table)
            .values(&row)
            .on_conflict(id)
            .do_update()
            .set(&row)
            .execute(conn);
    })
    .await?;
    Ok(())
}

pub async fn delete_session(pool: impl AsRef<Pool>, id_arg: &Id) -> Result<()> {
    let conn = pool.as_ref().get().await?;
    let id_arg = i128_to_uuid(id_arg.0);
    conn.interact(move |conn| delete(table.filter(id.eq(id_arg))).execute(conn))
        .await??;
    Ok(())
}
