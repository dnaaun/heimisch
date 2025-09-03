/// I'm not using https://docs.rs/serde-json-wasm/latest/serde_json_wasm/
/// because that library didn't play nice with jiff.
use super::*;
use utils::JustSend;
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

impl<R: Table> RawTableAccessTrait<R> for JustSend<idb::ObjectStore> {
    type RawDb = JustSend<idb::Database>;

    async fn get(&self, key: &R::Id) -> Result<Option<R>, Error> {
        let fut = idb::ObjectStore::get(self, idb::Query::Key(to_value(key)?))?.into_future();
        let item = JustSend::new(fut)
            .await?
            .map(|i| from_value(i))
            .transpose()?;
        Ok(item)
    }

    async fn get_all(&self) -> Result<Vec<R>, Error> {
        let fut = idb::ObjectStore::get_all(self, None, None)?.into_future();
        let items = JustSend::new(fut)
            .await?
            .into_iter()
            .map(|i| from_value(i))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(items)
    }

    async fn put(&self, item: &R) -> Result<(), Error> {
        let fut = idb::ObjectStore::put(self, &to_value(item)?, None)?.into_future();
        JustSend::new(fut).await?;
        Ok(())
    }

    async fn delete(&self, key: &R::Id) -> Result<(), Error> {
        let fut = idb::ObjectStore::delete(self, idb::Query::Key(to_value(key)?))?.into_future();
        JustSend::new(fut).await?;
        Ok(())
    }

    fn index(&self, name: &str) -> JustSend<idb::Index> {
        JustSend::new(
            idb::ObjectStore::index(self, name).expect("This rarely (never) happens I hope."),
        )
    }
}

impl RawTxnTrait for JustSend<idb::Transaction> {
    type RawDb = JustSend<idb::Database>;

    fn commit(self) -> Result<(), Error> {
        idb::Transaction::commit(self.take())?;
        Ok(())
    }

    fn abort(self) -> Result<(), Error> {
        idb::Transaction::abort(self.take())?;
        Ok(())
    }

    fn get_table<R: Table>(&self, store_name: &str) -> JustSend<idb::ObjectStore> {
        JustSend::new(
            idb::Transaction::object_store(self, store_name)
                .expect("This rarely (never) happens I hope."),
        )
    }
}

impl RawDbBuilderTrait for JustSend<idb::builder::DatabaseBuilder> {
    type RawDb = JustSend<idb::Database>;

    async fn build(self) -> Result<Self::RawDb, Error> {
        Ok(JustSend::new(
            idb::builder::DatabaseBuilder::build(self.take()).await?,
        ))
    }

    fn add_table(self, table_builder: JustSend<idb::builder::ObjectStoreBuilder>) -> Self {
        JustSend::new(idb::builder::DatabaseBuilder::add_object_store(
            self.take(),
            table_builder.take(),
        ))
    }
}

impl RawDbTrait for JustSend<idb::Database> {
    type Error = Error;
    type RawTxn = JustSend<idb::Transaction>;
    type RawDbBuilder = JustSend<idb::builder::DatabaseBuilder>;
    type RawTableBuilder = JustSend<idb::builder::ObjectStoreBuilder>;
    type RawIndex = JustSend<idb::Index>;
    type RawTableAccess<R: Table> = JustSend<idb::ObjectStore>;

    fn txn(&self, store_names: &[&str], read_write: bool) -> Self::RawTxn {
        JustSend::new(
            self.transaction(
                store_names,
                if read_write {
                    idb::TransactionMode::ReadWrite
                } else {
                    idb::TransactionMode::ReadOnly
                },
            )
            .expect("This rarely (never) happens I hope."),
        )
    }

    fn builder(name: &str) -> Self::RawDbBuilder {
        JustSend::new(idb::builder::DatabaseBuilder::new(name))
    }

    fn table_builder<R: Table>() -> Self::RawTableBuilder {
        let builder = idb::builder::ObjectStoreBuilder::new(R::NAME);
        JustSend::new(R::index_names().iter().fold(builder, |builder, name| {
            builder.add_index(idb::builder::IndexBuilder::new(
                name.to_string(),
                idb::KeyPath::Single(name.to_string()),
            ))
        }))
    }
}

impl RawIndexTrait for JustSend<idb::Index> {
    type RawDb = JustSend<idb::Database>;

    async fn get<IS: IndexSpec>(&self, value: &IS::Type) -> Result<Option<IS::Table>, Error> {
        Ok(idb::Index::get(self, idb::Query::Key(to_value(value)?))?
            .await?
            .map(|i| from_value(i))
            .transpose()?)
    }

    async fn get_all<IS: IndexSpec>(
        &self,
        value: Option<&IS::Type>,
    ) -> Result<Vec<IS::Table>, Error> {
        Ok(idb::Index::get_all(
            self,
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
