use std::ops::Deref;

/// I'm not using https://docs.rs/serde-json-wasm/latest/serde_json_wasm/
/// because that library didn't play nice with jiff.
use super::*;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum Error {
    Js(JsValue),
    Serde(serde_json::Error),
    Id(idb::Error),
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::Serde(value)
    }
}

impl From<idb::Error> for Error {
    fn from(value: idb::Error) -> Self {
        Error::Id(value)
    }
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Error::Js(value)
    }
}

pub fn from_value<T>(value: JsValue) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let s = js_sys::JSON::stringify(&value)?;
    let s = s.as_string().unwrap();
    Ok(serde_json::from_str(&s)?)
}

pub fn to_value<T>(t: &T) -> Result<JsValue, Error>
where
    T: Serialize,
{
    let s = serde_json::to_string(t)?;
    Ok(js_sys::JSON::parse(&s)?)
}

impl<R: Table> RawTableAccessTrait<R> for SendWrapper<idb::ObjectStore> {
    type RawDb = SendWrapper<idb::Database>;

    async fn get(&self, key: &R::Id) -> Result<Option<R>, Error> {
        let item = self
            .get(idb::Query::Key(to_value(key)?))?
            .await?
            .map(|i| from_value(i))
            .transpose()?;
        Ok(item)
    }

    async fn get_all(&self) -> Result<Vec<R>, Error> {
        let items = idb::ObjectStore::get_all(self, None, None)?
            .await?
            .into_iter()
            .map(|i| from_value(i))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(items)
    }

    async fn put(&self, item: &R) -> Result<(), Error> {
        idb::ObjectStore::put(self, &to_value(item)?, None)?.await?;
        Ok(())
    }

    async fn delete(&self, key: &R::Id) -> Result<(), Error> {
        idb::ObjectStore::delete(self, idb::Query::Key(to_value(key)?))?.await?;
        Ok(())
    }

    fn index(&self, name: &str) -> idb::Index {
        idb::ObjectStore::index(self, name).expect("This rarely (never) happens I hope.")
    }
}

impl RawTxnTrait for SendWrapper<idb::Transaction> {
    type RawDb = SendWrapper<idb::Database>;

    fn commit(self) -> Result<(), Error> {
        idb::Transaction::commit(self)?;
        Ok(())
    }

    fn abort(self) -> Result<(), Error> {
        idb::Transaction::abort(self)?;
        Ok(())
    }

    fn get_table<R: Table>(&self, store_name: &str) -> SendWrapper<idb::ObjectStore> {
        SendWrapper::new(
            idb::Transaction::object_store(self, store_name)
                .expect("This rarely (never) happens I hope."),
        )
    }
}

impl RawDbBuilderTrait for SendWrapper<idb::builder::DatabaseBuilder> {
    type RawDb = SendWrapper<idb::Database>;

    async fn build(self) -> Result<Self::RawDb, Error> {
        Ok(SendWrapper::new(
            idb::builder::DatabaseBuilder::build(self.take()).await?,
        ))
    }

    fn add_table(self, table_builder: SendWrapper<idb::builder::ObjectStoreBuilder>) -> Self {
        SendWrapper::new(idb::builder::DatabaseBuilder::add_object_store(
            self.take(),
            table_builder.take(),
        ))
    }
}

impl RawDbTrait for SendWrapper<idb::Database> {
    type Error = Error;
    type RawTxn = SendWrapper<idb::Transaction>;
    type RawDbBuilder = SendWrapper<idb::builder::DatabaseBuilder>;
    type RawTableBuilder = SendWrapper<idb::builder::ObjectStoreBuilder>;
    type RawIndex = SendWrapper<idb::Index>;
    type RawTableAccess<R: Table> = SendWrapper<idb::ObjectStore>;

    fn txn(&self, store_names: &[&str], read_write: bool) -> Self::RawTxn {
        self.transaction(
            store_names,
            if read_write {
                idb::TransactionMode::ReadWrite
            } else {
                idb::TransactionMode::ReadOnly
            },
        )
        .expect("This rarely (never) happens I hope.")
    }

    fn builder(name: &str) -> Self::RawDbBuilder {
        idb::builder::DatabaseBuilder::new(name)
    }

    fn table_builder<R: Table>() -> Self::RawTableBuilder {
        let builder = idb::builder::ObjectStoreBuilder::new(R::NAME);
        R::index_names().iter().fold(builder, |builder, name| {
            builder.add_index(idb::builder::IndexBuilder::new(
                name.to_string(),
                idb::KeyPath::Single(name.to_string()),
            ))
        })
    }
}

impl RawIndexTrait for idb::Index {
    type RawDb = idb::Database;

    async fn get<IS: IndexSpec>(&self, value: &IS::Type) -> Result<Option<IS::Table>, Error> {
        Ok(self
            .get(idb::Query::Key(to_value(value)?))?
            .await?
            .map(|i| from_value(i))
            .transpose()?)
    }

    async fn get_all<IS: IndexSpec>(
        &self,
        value: Option<&IS::Type>,
    ) -> Result<Vec<IS::Table>, Error> {
        Ok(self
            .get_all(
                value
                    .map(|v| to_value(v).map(idb::Query::Key))
                    .transpose()?,
                None,
            )?
            .await?
            .into_iter()
            .map(from_value)
            .collect::<Result<Vec<_>, _>>()?)
    }
}
