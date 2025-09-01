use rusqlite::{Connection, params};
use std::sync::{Arc, Mutex};

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

impl<R: Table> RawTableAccessTrait<R> for SqliteTableAccess {
    type RawDb = SqliteDatabase;

    async fn get(&self, id: &R::Id) -> Result<Option<R>, Error> {
        let conn = self.connection.lock().unwrap();
        let id_json = serde_json::to_string(id)?;

        let mut stmt = conn.prepare(&format!(
            "SELECT data FROM {} WHERE id = ?",
            self.table_name
        ))?;
        let mut rows = stmt.query_map([id_json], |row| {
            let data: String = row.get(0)?;
            Ok(data)
        })?;

        if let Some(row) = rows.next() {
            let data = row?;
            let item: R = serde_json::from_str(&data)?;
            Ok(Some(item))
        } else {
            Ok(None)
        }
    }

    async fn get_all(&self) -> Result<Vec<R>, Error> {
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare(&format!("SELECT data FROM {}", self.table_name))?;
        let rows = stmt.query_map([], |row| {
            let data: String = row.get(0)?;
            Ok(data)
        })?;

        let mut items = Vec::new();
        for row in rows {
            let data = row?;
            let item: R = serde_json::from_str(&data)?;
            items.push(item);
        }
        Ok(items)
    }

    async fn put(&self, item: &R) -> Result<(), Error> {
        let conn = self.connection.lock().unwrap();
        let id_json = serde_json::to_string(item.id())?;
        let data_json = serde_json::to_string(item)?;

        conn.execute(
            &format!(
                "INSERT OR REPLACE INTO {} (id, data) VALUES (?, ?)",
                self.table_name
            ),
            params![id_json, data_json],
        )?;
        Ok(())
    }

    async fn delete(&self, id: &R::Id) -> Result<(), Error> {
        let conn = self.connection.lock().unwrap();
        let id_json = serde_json::to_string(id)?;

        conn.execute(
            &format!("DELETE FROM {} WHERE id = ?", self.table_name),
            params![id_json],
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
    connection: Arc<Mutex<Connection>>,
    in_transaction: bool,
}

impl SqliteTransaction {
    fn new(connection: Arc<Mutex<Connection>>) -> Result<Self, Error> {
        {
            let conn = connection.lock().unwrap();
            conn.execute("BEGIN TRANSACTION", [])?;
        }
        Ok(Self {
            connection,
            in_transaction: true,
        })
    }
}

impl RawTxnTrait for SqliteTransaction {
    type RawDb = SqliteDatabase;

    fn commit(mut self) -> Result<(), Error> {
        if self.in_transaction {
            let conn = self.connection.lock().unwrap();
            conn.execute("COMMIT", [])?;
            self.in_transaction = false;
        }
        Ok(())
    }

    fn abort(mut self) -> Result<(), Error> {
        if self.in_transaction {
            let conn = self.connection.lock().unwrap();
            conn.execute("ROLLBACK", [])?;
            self.in_transaction = false;
        }
        Ok(())
    }

    fn get_table<R: Table>(&self, store_name: &str) -> SqliteTableAccess {
        SqliteTableAccess::new(self.connection.clone(), store_name.to_string())
    }
}

impl Drop for SqliteTransaction {
    fn drop(&mut self) {
        if self.in_transaction {
            let _ = self.connection.lock().unwrap().execute("ROLLBACK", []);
        }
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
    type RawTableAccess<R: Table> = SqliteTableAccess;

    fn txn(&self, _store_names: &[&str], _read_write: bool) -> Self::RawTxn {
        SqliteTransaction::new(self.connection.clone()).expect("Failed to create transaction")
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

    async fn get<IS: IndexSpec>(&self, value: &IS::Type) -> Result<Option<IS::Table>, Error> {
        let conn = self.connection.lock().unwrap();
        let value_json = serde_json::to_string(value)?;

        let mut stmt = conn.prepare(&format!(
            "SELECT data FROM {} WHERE json_extract(data, '$.{}') = ?",
            self.table_name, self.index_name
        ))?;

        let mut rows = stmt.query_map([value_json], |row| {
            let data: String = row.get(0)?;
            Ok(data)
        })?;

        if let Some(row) = rows.next() {
            let data = row?;
            let item: IS::Table = serde_json::from_str(&data)?;
            Ok(Some(item))
        } else {
            Ok(None)
        }
    }

    async fn get_all<IS: IndexSpec>(
        &self,
        value: Option<&IS::Type>,
    ) -> Result<Vec<IS::Table>, Error> {
        let conn = self.connection.lock().unwrap();

        let mut items = Vec::new();

        if let Some(value) = value {
            let value_json = serde_json::to_string(value)?;
            let query = format!(
                "SELECT data FROM {} WHERE json_extract(data, '$.{}') = ?",
                self.table_name, self.index_name
            );
            let mut stmt = conn.prepare(&query)?;
            let rows = stmt.query_map([value_json], |row| {
                let data: String = row.get(0)?;
                Ok(data)
            })?;

            for row in rows {
                let data = row?;
                let item: IS::Table = serde_json::from_str(&data)?;
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
                let item: IS::Table = serde_json::from_str(&data)?;
                items.push(item);
            }
        }
        Ok(items)
    }
}
