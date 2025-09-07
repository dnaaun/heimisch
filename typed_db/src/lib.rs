#[cfg(feature = "idb")]
pub mod idb_impl;
pub mod raw_traits;
#[cfg(feature = "sqlite")]
pub mod sqlite_impl;

pub use derivative::Derivative;
use futures::{
    SinkExt, StreamExt, TryStreamExt,
    channel::{mpsc, oneshot},
    lock::Mutex,
    stream,
};
pub use raw_traits::RawDbTrait;
use raw_traits::*;
use utils::spawn;

use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    marker::PhantomData,
    sync::Arc,
};

use serde::{Serialize, de::DeserializeOwned};

pub trait Table: DeserializeOwned + Serialize + Clone + 'static + Send + Sync + Sized {
    const NAME: &'static str;

    /// I need the `Serialize` here to make it easier to integrate with idb and for optimistic stuff.
    /// I need the `DeserializeOwned` here for optimistic stuff.
    type Id: serde::Serialize
        + DeserializeOwned
        // There's somewhere where we clone the id and also 'static is required there.
        + Clone
        + 'static
        + Send
        + Sync;

    fn id(&self) -> &Self::Id;

    fn index_names() -> &'static [&'static str];
}

#[derive(Clone, Debug)]
enum WriteData {
    Put {
        id: SerializedId,
        row: SerializedObject,
        table_name: &'static str,
    },
    Delete {
        id: SerializedId,
        table_name: &'static str,
    },
}

impl WriteData {
    pub fn table_name(&self) -> &'static str {
        match self {
            WriteData::Put { table_name, .. } => table_name,
            WriteData::Delete { table_name, .. } => table_name,
        }
    }
}

enum ReadData {
    Get {
        id: SerializedId,
        table_name: &'static str,
    },
    GetAll {
        table_name: &'static str,
    },
    IndexGet {
        value: SerializedValue,
        index_name: &'static str,
        table_name: &'static str,
    },
    IndexGetAll {
        value: Option<SerializedValue>,
        index_name: &'static str,
        table_name: &'static str,
    },
}

impl ReadData {
    pub fn table_name(&self) -> &'static str {
        match self {
            ReadData::Get { table_name, .. } => table_name,
            ReadData::GetAll { table_name, .. } => table_name,
            ReadData::IndexGet { table_name, .. } => table_name,
            ReadData::IndexGetAll { table_name, .. } => table_name,
        }
    }
}

enum TxnData<Error> {
    Write {
        data: Vec<WriteData>,
        sender: oneshot::Sender<Result<(), Error>>,
    },
    Read {
        data: ReadData,
        sender: oneshot::Sender<Result<ReadResponse, Error>>,
    },
}

#[derive(Debug)]
enum ReadResponse {
    Row(Option<SerializedObject>),
    Rows(Vec<SerializedObject>),
}

impl ReadResponse {
    pub fn unwrap_row(self) -> Option<SerializedObject> {
        match self {
            ReadResponse::Row(row) => row,
            _ => panic!("Expected a row, got a rows"),
        }
    }

    pub fn unwrap_rows(self) -> Vec<SerializedObject> {
        match self {
            ReadResponse::Rows(rows) => rows,
            _ => panic!("Expected rows, got a row"),
        }
    }
}

pub struct Db<RawDb: RawDbTrait> {
    // So sqlite has a cocurrency limit of 1 for transactions. The first thing I tried
    // was locking a mutex for the duration of transaction (ie, until the `Txn` is dropped).
    // I opted to go for an async
    // Mutex. And the issue I ran into there was imagine the following sequence of events:
    //
    //  ```rust
    //  let txn1 = db.txn().await;
    //  let txn2 = db.txn().await; // This will never resolve because the mutex is locked.
    //  txn1.commit().await;
    //  ```
    // I think the same issue would happen with a sync Mutex as well.
    //
    // So I thought of two options:
    // 1. Impose a programming constraint on myself of "avoid concurrent transactions":
    //    But that's hard to get right and easy to get wrong (and perhaps impossible to
    //    totally avoid, as I will have websocket updates requiring db updates randomly).
    // 2. Implement a queue system with a worker task processing the queue.
    event_queue_sender: mpsc::Sender<TxnData<RawDb::Error>>,
    event_queue_kill_switch_sender: Option<oneshot::Sender<()>>,
}

impl<RawDb: RawDbTrait> Drop for Db<RawDb> {
    fn drop(&mut self) {
        match self
            .event_queue_kill_switch_sender
            .take()
            .expect("Should be present")
            .send(())
        {
            Ok(_) => (),
            Err(_) => {
                println!("Failed to send kill switch to event queue.");
            }
        }
    }
}

pub struct DbBuilder<RawDb: RawDbTrait> {
    name: String,
    table_builders: HashMap<TypeId, RawDb::RawTableBuilder>,
}

impl<RawDb: RawDbTrait> Db<RawDb> {
    pub fn builder(name: String) -> DbBuilder<RawDb> {
        DbBuilder {
            name,
            table_builders: Default::default(),
        }
    }
}

/// We drain the event queue when the kiill switch is invoked, but it's still possible
/// that some things are not processed if things are added to the queue.
fn handle_events<RawDb: RawDbTrait>(
    mut event_queue_receiver: mpsc::Receiver<TxnData<RawDb::Error>>,
    mut event_queue_kill_switch: oneshot::Receiver<()>,
    raw_db: Arc<RawDb>,
) -> impl Future<Output = ()> + Send + Sync {
    async move {
        let mut kill_switch_invoked = false;
        while !kill_switch_invoked {
            let requests = futures::select! {
                request = event_queue_receiver.next() => vec![request],
                _ = event_queue_kill_switch => {
                    kill_switch_invoked = true;
                    event_queue_receiver.close();
                    std::iter::from_fn(|| event_queue_receiver.try_next().ok()).collect::<Vec<_>>()
                }
            };

            for request in requests {
                let request = match request {
                    Some(request) => request,
                    None => {
                        tracing::error!(
                            "Not sure why this would happen actually. I presume all clones of the Sender have been dropped?"
                        );
                        break;
                    }
                };

                match request {
                    TxnData::Write { data, sender: done } => {
                        let table_names = data
                            .iter()
                            .map(|data| data.table_name())
                            .collect::<Vec<_>>();
                        let txn = raw_db.txn(&table_names, true).await;
                        let results = stream::iter(data)
                            .then(async |data| match data {
                                WriteData::Put {
                                    id,
                                    row,
                                    table_name,
                                } => txn.get_table(table_name).put(&id, &row).await,
                                WriteData::Delete { id, table_name } => {
                                    txn.get_table(table_name).delete(&id).await
                                }
                            })
                            .try_collect::<()>()
                            .await;

                        if let Err(e) = txn.commit().await {
                            done.send(Err(e)).unwrap();
                        } else {
                            done.send(results).unwrap();
                        }
                    }
                    TxnData::Read {
                        data,
                        sender: result,
                    } => {
                        let txn = raw_db.txn(&[data.table_name()], false).await;
                        let response = async || {
                            let result = match data {
                                ReadData::Get { id, table_name } => {
                                    ReadResponse::Row(txn.get_table(table_name).get(&id).await?)
                                }
                                ReadData::GetAll { table_name } => {
                                    ReadResponse::Rows(txn.get_table(table_name).get_all().await?)
                                }
                                ReadData::IndexGet {
                                    value,
                                    table_name,
                                    index_name,
                                } => ReadResponse::Row(
                                    txn.get_table(table_name)
                                        .index(index_name)
                                        .get(&value)
                                        .await?,
                                ),
                                ReadData::IndexGetAll {
                                    value,
                                    table_name,
                                    index_name,
                                } => ReadResponse::Rows(
                                    txn.get_table(table_name)
                                        .index(index_name)
                                        .get_all(value.as_ref())
                                        .await?,
                                ),
                            };
                            txn.commit().await?;
                            Ok::<_, RawDb::Error>(result)
                        };

                        result.send(response().await).unwrap();
                    }
                }
            }
        }
    }
}

impl<RawDb: RawDbTrait> DbBuilder<RawDb> {
    pub fn with_table<R: Table + 'static>(mut self) -> DbBuilder<RawDb> {
        self.table_builders
            .insert(TypeId::of::<R>(), RawDb::table_builder::<R>());
        DbBuilder {
            name: self.name,
            table_builders: self.table_builders,
        }
    }

    pub async fn build(self) -> Result<Db<RawDb>, RawDb::Error> {
        let db_builder = self.table_builders.into_iter().fold(
            RawDb::builder(&self.name),
            |db_builder, (_, table_builder)| db_builder.add_table(table_builder),
        );

        let raw_db = Arc::new(db_builder.build().await?);

        let (event_queue_sender, event_queue_receiver) = mpsc::channel(100);
        let (event_queue_kill_switch_sender, event_queue_kill_switch_receiver) = oneshot::channel();

        tokio::task::spawn(handle_events(
            event_queue_receiver,
            event_queue_kill_switch_receiver,
            raw_db.clone(),
        ));

        Ok(Db {
            event_queue_sender,
            event_queue_kill_switch_sender: Some(event_queue_kill_switch_sender),
        })
    }
}

impl<RawDb: RawDbTrait> Db<RawDb> {
    pub fn txn(&self) -> TxnBuilder<'_, RawDb, ReadOnly> {
        TxnBuilder {
            event_queue_sender: self.event_queue_sender.clone(),
            store_names: Default::default(),
            db: self,
            mode: PhantomData,
        }
    }
}

pub struct Present;

pub struct ReadOnly;
pub struct ReadWrite;

pub trait TxnMode {
    const IS_READ_WRITE: bool;

    // I need this because Rust's support for constraining on associated consts is incomplete.
    // See #92827 <https://github.com/rust-lang/rust/issues/92827>
    type SupportsReadWrite;
}

impl TxnMode for ReadOnly {
    const IS_READ_WRITE: bool = false;
    type SupportsReadWrite = ();
}

impl TxnMode for ReadWrite {
    const IS_READ_WRITE: bool = true;
    type SupportsReadWrite = Present;
}
pub struct TxnBuilder<'db, RawDb: RawDbTrait, Mode> {
    db: &'db Db<RawDb>,
    store_names: HashSet<&'static str>,
    mode: PhantomData<Mode>,
    event_queue_sender: mpsc::Sender<TxnData<RawDb::Error>>,
}

impl<'db, RawDb: RawDbTrait, Mode> TxnBuilder<'db, RawDb, Mode>
where
    Mode: TxnMode,
{
    pub fn with_table<R: Table + 'static>(self) -> TxnBuilder<'db, RawDb, Mode> {
        let mut new_table_names = self.store_names;
        new_table_names.insert(&R::NAME);

        TxnBuilder {
            store_names: new_table_names,
            ..self
        }
    }

    pub fn read_write(self) -> TxnBuilder<'db, RawDb, ReadWrite> {
        TxnBuilder {
            store_names: self.store_names,
            db: self.db,
            mode: PhantomData,
            event_queue_sender: self.event_queue_sender,
        }
    }

    pub fn read_only(self) -> TxnBuilder<'db, RawDb, ReadOnly> {
        TxnBuilder {
            store_names: self.store_names,
            db: self.db,
            mode: PhantomData,
            event_queue_sender: self.event_queue_sender,
        }
    }
}
impl<'db, Db: RawDbTrait, Mode> TxnBuilder<'db, Db, Mode>
where
    Mode: TxnMode,
{
    pub async fn build(self) -> Txn<Db, Mode> {
        Txn {
            _mode: PhantomData,
            write_data: Arc::new(Mutex::new(Vec::new())),
            event_queue_sender: self.event_queue_sender,
        }
    }
}

pub struct Txn<Db: RawDbTrait, Mode> {
    _mode: PhantomData<Mode>,
    write_data: Arc<Mutex<Vec<WriteData>>>,
    event_queue_sender: mpsc::Sender<TxnData<Db::Error>>,
}

impl<Db: RawDbTrait, Mode> Txn<Db, Mode> {
    pub fn table<R>(&self) -> TableAccess<Db, R, Mode>
    where
        R: Table,
        Mode: TxnMode,
    {
        TableAccess {
            event_queue_sender: self.event_queue_sender.clone(),
            mode: PhantomData,
            write_data: Some(self.write_data.clone()),
        }
    }

    pub async fn commit(self) -> Result<(), Db::Error> {
        Self::commit_impl(self.write_data.clone(), self.event_queue_sender).await
    }

    async fn commit_impl(
        write_data: Arc<Mutex<Vec<WriteData>>>,
        mut event_queue_sender: mpsc::Sender<TxnData<Db::Error>>,
    ) -> Result<(), Db::Error> {
        let write_data = write_data.lock().await;
        let (sender, receiver) = oneshot::channel();
        event_queue_sender
            .send(TxnData::Write {
                data: write_data.clone(),
                sender,
            })
            .await
            .unwrap();
        receiver.await.unwrap()?;
        Ok(())
    }

    pub async fn abort(self) -> Result<(), Db::Error> {
        self.write_data.lock().await.clear();
        Ok(())
    }
}

impl<Db: RawDbTrait, Mode> Txn<Db, Mode>
where
    Mode: 'static,
{
    pub fn drop(self) {
        spawn(Self::commit_impl(
            self.write_data.clone(),
            self.event_queue_sender,
        ));
    }
}

#[derive(Derivative)]
#[derivative(Clone(bound = ""))]
pub struct TableAccess<RawDb: RawDbTrait, R: Table, Mode> {
    pub(crate) event_queue_sender: mpsc::Sender<TxnData<RawDb::Error>>,
    pub(crate) mode: PhantomData<(R, Mode)>,

    /// Will be non-None for read-write transactions.
    write_data: Option<Arc<Mutex<Vec<WriteData>>>>,
}

impl<Db: RawDbTrait, R: Table, Mode> TableAccess<Db, R, Mode>
where
    Mode: TxnMode,
{
    pub async fn get(&self, id: &R::Id) -> Result<Option<R>, Db::Error> {
        let id = SerializedId::new_from_id::<R>(id);
        let (sender, receiver) = oneshot::channel();
        let txn_data = TxnData::Read {
            data: ReadData::Get {
                id,
                table_name: R::NAME,
            },
            sender,
        };
        self.event_queue_sender
            .clone()
            .send(txn_data)
            .await
            .unwrap();

        Ok(receiver
            .await
            .unwrap()?
            .unwrap_row()
            .map(|row| serde_json::from_str(&row).unwrap()))
    }

    pub async fn get_all(&self) -> Result<Vec<R>, Db::Error> {
        let (sender, receiver) = oneshot::channel();
        let txn_data = TxnData::Read {
            data: ReadData::GetAll {
                table_name: R::NAME,
            },
            sender,
        };
        self.event_queue_sender
            .clone()
            .send(txn_data)
            .await
            .unwrap();
        Ok(receiver
            .await
            .unwrap()?
            .unwrap_rows()
            .into_iter()
            .map(|row| serde_json::from_str(&row).unwrap())
            .collect())
    }

    pub fn index<IS: IndexSpec<Table = R>>(&self) -> Index<Db, IS> {
        Index {
            event_queue_sender: self.event_queue_sender.clone(),
            _spec: PhantomData,
        }
    }
}

impl<Db: RawDbTrait, R: Table, Mode> TableAccess<Db, R, Mode>
where
    Mode: TxnMode<SupportsReadWrite = Present>,
{
    pub async fn put(&self, item: &R) -> Result<(), Db::Error> {
        self.write_data
            .as_ref()
            .expect("Should be Some for write transactions")
            .lock()
            .await
            .push(WriteData::Put {
                id: SerializedId::new_from_id::<R>(item.id()),
                row: SerializedObject::from_row(item).unwrap(),
                table_name: R::NAME,
            });
        Ok(())
    }

    pub async fn delete(&self, id: &R::Id) -> Result<(), Db::Error> {
        self.write_data
            .as_ref()
            .expect("Should be Some for write transactions")
            .lock()
            .await
            .push(WriteData::Delete {
                id: SerializedId::new_from_id::<R>(id),
                table_name: R::NAME,
            });
        Ok(())
    }
}

pub trait IndexSpec {
    type Table: Table;
    const NAME: &'static str;

    // The `Eq` requirement is used when doing optimistic updates, and it's not really
    // unrealistic at all to expect things that indexed by indexed db have a `Eq` Rust
    // representation.
    type Type: Serialize + Eq;

    fn get_index_value(row: &Self::Table) -> &Self::Type;
}

pub struct Index<RawDb: RawDbTrait, IS: IndexSpec> {
    pub(crate) event_queue_sender: mpsc::Sender<TxnData<RawDb::Error>>,
    _spec: PhantomData<IS>,
}

impl<RawDb: RawDbTrait, IS: IndexSpec> Index<RawDb, IS> {
    pub async fn get(
        &self,
        value: &IS::Type,
    ) -> Result<Option<IS::Table>, <RawDb as RawDbTrait>::Error> {
        let (sender, receiver) = oneshot::channel();
        let txn_data = TxnData::Read {
            data: ReadData::IndexGet {
                value: SerializedValue::from_value(value).unwrap(),
                index_name: IS::NAME,
                table_name: IS::Table::NAME,
            },
            sender,
        };
        self.event_queue_sender
            .clone()
            .send(txn_data)
            .await
            .unwrap();
        Ok(receiver
            .await
            .unwrap()?
            .unwrap_row()
            .map(|row| serde_json::from_str(&row).unwrap()))
    }

    pub async fn get_all(
        &self,
        value: Option<&IS::Type>,
    ) -> Result<Vec<IS::Table>, <RawDb as RawDbTrait>::Error> {
        let (sender, receiver) = oneshot::channel();
        let txn_data = TxnData::Read {
            data: ReadData::IndexGetAll {
                value: value.map(|v| SerializedValue::from_value(v).unwrap()),
                index_name: IS::NAME,
                table_name: IS::Table::NAME,
            },
            sender,
        };
        self.event_queue_sender
            .clone()
            .send(txn_data)
            .await
            .unwrap();
        Ok(receiver
            .await
            .unwrap()?
            .unwrap_rows()
            .into_iter()
            .map(|row| serde_json::from_str(&row).unwrap())
            .collect())
    }
}
