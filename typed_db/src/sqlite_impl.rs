use futures::lock::Mutex;
use rusqlite::{Connection, params};
use std::sync::Arc;

use super::*;

#[derive(Debug)]
pub enum Error {
    Sqlite(rusqlite::Error),
    Serde(serde_json::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(value: rusqlite::Error) -> Self {
        Error::Sqlite(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::Serde(value)
    }
}

pub struct SqliteTableAccess {
    connection: Arc<Mutex<Connection>>,
    table_name: String,
}

impl SqliteTableAccess {
    fn new(connection: Arc<Mutex<Connection>>, table_name: String) -> Self {
        Self {
            connection,
            table_name,
        }
    }
}

impl RawTableAccessTrait for SqliteTableAccess {
    type RawDb = SqliteDatabase;

    async fn get(&self, id: &SerializedId) -> Result<Option<SerializedObject>, Error> {
        let conn = self.connection.lock().await;
        let id_str = id.0.clone();

        let mut stmt = conn.prepare(&format!(
            "SELECT data FROM {} WHERE id = ?",
            self.table_name
        ))?;
        let mut rows = stmt.query_map([id_str], |row| {
            let data: String = row.get(0)?;
            Ok(data)
        })?;

        if let Some(row) = rows.next() {
            let data = row?;
            Ok(Some(SerializedObject(data)))
        } else {
            Ok(None)
        }
    }

    async fn get_all(&self) -> Result<Vec<SerializedObject>, Error> {
        let conn = self.connection.lock().await;
        let mut stmt = conn.prepare(&format!("SELECT data FROM {}", self.table_name))?;
        let rows = stmt.query_map([], |row| {
            let data: String = row.get(0)?;
            Ok(SerializedObject(data))
        })?;

        let items: Result<Vec<_>, _> = rows.into_iter().collect();
        Ok(items?)
    }

    async fn put(&self, id: &SerializedId, item: &SerializedObject) -> Result<(), Error> {
        let conn = self.connection.lock().await;

        conn.execute(
            &format!(
                "INSERT OR REPLACE INTO {} (id, data) VALUES (?, ?)",
                self.table_name
            ),
            params![id, item],
        )?;
        Ok(())
    }

    async fn delete(&self, id: &SerializedId) -> Result<(), Error> {
        let conn = self.connection.lock().await;

        conn.execute(
            &format!("DELETE FROM {} WHERE id = ?", self.table_name),
            params![id],
        )?;
        Ok(())
    }

    fn index(&self, name: &str) -> SqliteIndex {
        SqliteIndex::new(
            self.connection.clone(),
            self.table_name.clone(),
            name.to_string(),
        )
    }
}

pub struct SqliteTransaction {
    conn: Arc<Mutex<Connection>>,
    in_transaction: bool,
}

impl SqliteTransaction {
    async fn new(conn: Arc<Mutex<Connection>>) -> Self {
        conn.lock().await.execute("BEGIN TRANSACTION", []).unwrap(); // I should make new() fallible I suppose.
        Self {
            conn,
            in_transaction: true,
        }
    }
}

impl RawTxnTrait for SqliteTransaction {
    type RawDb = SqliteDatabase;

    async fn commit(mut self) -> Result<(), Error> {
        if self.in_transaction {
            self.conn.lock().await.execute("COMMIT", [])?;
            self.in_transaction = false;
        }
        Ok(())
    }

    async fn abort(mut self) -> Result<(), Error> {
        if self.in_transaction {
            let conn = self.conn.lock().await;
            conn.execute("ROLLBACK", [])?;
            self.in_transaction = false;
        }
        Ok(())
    }

    fn get_table(&self, table_name: &str) -> SqliteTableAccess {
        SqliteTableAccess::new(self.conn.clone(), table_name.to_string())
    }
}

pub struct SqliteDatabaseBuilder {
    name: String,
    table_builders: Vec<SqliteTableBuilder>,
}

impl SqliteDatabaseBuilder {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            table_builders: Vec::new(),
        }
    }
}

impl RawDbBuilderTrait for SqliteDatabaseBuilder {
    type RawDb = SqliteDatabase;

    async fn build(self) -> Result<Self::RawDb, Error> {
        let conn = Connection::open(&self.name)?;

        for table_builder in &self.table_builders {
            conn.execute(
                &format!(
                    "CREATE TABLE IF NOT EXISTS {} (id TEXT PRIMARY KEY, data TEXT)",
                    table_builder.table_name
                ),
                [],
            )?;

            for index_name in &table_builder.index_names {
                conn.execute(
                    &format!(
                        "CREATE INDEX IF NOT EXISTS idx_{}_{} ON {} (json_extract(data, '$.{}'))",
                        table_builder.table_name, index_name, table_builder.table_name, index_name
                    ),
                    [],
                )?;
            }
        }

        Ok(SqliteDatabase {
            connection: Arc::new(Mutex::new(conn)),
        })
    }

    fn add_table(mut self, table_builder: SqliteTableBuilder) -> Self {
        self.table_builders.push(table_builder);
        self
    }
}

pub struct SqliteDatabase {
    connection: Arc<Mutex<Connection>>,
}

impl RawDbTrait for SqliteDatabase {
    type Error = Error;
    type RawTxn = SqliteTransaction;
    type RawDbBuilder = SqliteDatabaseBuilder;
    type RawIndex = SqliteIndex;
    type RawTableBuilder = SqliteTableBuilder;
    type RawTableAccess = SqliteTableAccess;

    async fn txn(&self, _store_names: &[&str], _read_write: bool) -> Self::RawTxn {
        SqliteTransaction::new(self.connection.clone()).await
    }

    fn builder(name: &str) -> Self::RawDbBuilder {
        SqliteDatabaseBuilder::new(name)
    }

    fn table_builder<R: Table>() -> Self::RawTableBuilder {
        SqliteTableBuilder {
            table_name: R::NAME.to_string(),
            index_names: R::index_names().iter().map(|s| s.to_string()).collect(),
        }
    }
}

pub struct SqliteTableBuilder {
    table_name: String,
    index_names: Vec<String>,
}

pub struct SqliteIndex {
    connection: Arc<Mutex<Connection>>,
    table_name: String,
    index_name: String,
}

impl SqliteIndex {
    fn new(connection: Arc<Mutex<Connection>>, table_name: String, index_name: String) -> Self {
        Self {
            connection,
            table_name,
            index_name,
        }
    }
}

impl RawIndexTrait for SqliteIndex {
    type RawDb = SqliteDatabase;

    async fn get(&self, value: &SerializedValue) -> Result<Option<SerializedObject>, Error> {
        let conn = self.connection.lock().await;
        let value_str = serde_json::to_string(value)?;

        let mut stmt = conn.prepare(&format!(
            "SELECT data FROM {} WHERE json_extract(data, '$.{}') = ?",
            self.table_name, self.index_name
        ))?;

        let mut rows = stmt.query_map([value_str], |row| {
            let data: String = row.get(0)?;
            Ok(data)
        })?;

        if let Some(row) = rows.next() {
            let data = row?;
            let item = serde_json::from_str(&data)?;
            Ok(Some(item))
        } else {
            Ok(None)
        }
    }

    async fn get_all(
        &self,
        value: Option<&SerializedValue>,
    ) -> Result<Vec<SerializedObject>, Error> {
        let conn = self.connection.lock().await;

        let mut items = Vec::new();

        if let Some(value) = value {
            let value_str = serde_json::to_string(value)?;
            let query = format!(
                "SELECT data FROM {} WHERE json_extract(data, '$.{}') = ?",
                self.table_name, self.index_name
            );
            let mut stmt = conn.prepare(&query)?;
            let rows = stmt.query_map([value_str], |row| {
                let data: String = row.get(0)?;
                Ok(data)
            })?;

            for row in rows {
                let data = row?;
                let item = serde_json::from_str(&data)?;
                items.push(item);
            }
        } else {
            let query = format!("SELECT data FROM {}", self.table_name);
            let mut stmt = conn.prepare(&query)?;
            let rows = stmt.query_map([], |row| {
                let data: String = row.get(0)?;
                Ok(data)
            })?;

            for row in rows {
                let data = row?;
                let item = serde_json::from_str(&data)?;
                items.push(item);
            }
        }
        Ok(items)
    }
}
