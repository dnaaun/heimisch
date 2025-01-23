// use std::rc::Rc;
//
// use shared::{
//     sync_engine::storage,
//     types::{repository::Repository, user::User},
// };
// use typesafe_idb::{StoreMarker, TypesafeDb};
//
// pub struct DbBuilder {
//     inner: typesafe_idb::TypesafeDbBuilder<(
//         shared::types::repository::RepositoryStoreMarker,
//         (shared::types::user::UserStoreMarker, ()),
//     )>,
// }
//
// fn reactivity_trackers_idb_to_sync_engine(
//     _trackers: &typesafe_idb::ReactivityTrackers,
// ) -> storage::ReactivityTrackers {
//     todo!()
// }
//
// #[derive(Default)]
// struct ConvertMarker<T>(T);
//
// impl<M, S> storage::TableMarkerFor<S> for ConvertMarker<M> where M: typesafe_idb::StoreMarker<S> {}
//
// impl storage::DbBuilder for DbBuilder {
//     type Error = typesafe_idb::Error;
//
//     type Db = Db<(
//         shared::types::repository::RepositoryStoreMarker,
//         (shared::types::user::UserStoreMarker, ()),
//     )>;
//
//     async fn new() -> Self {
//         Self {
//             inner: TypesafeDb::builder("heimisch".into())
//                 .with_store::<User>()
//                 .with_store::<Repository>(),
//         }
//     }
//
//     fn with_commit_listener(
//         self,
//         commit_listener: std::rc::Rc<dyn Fn(&storage::ReactivityTrackers)>,
//     ) -> Self {
//         Self {
//             inner: self.inner.with_commit_listener(Rc::new(move |x| {
//                 commit_listener.clone()(&reactivity_trackers_idb_to_sync_engine(x))
//             })),
//         }
//     }
//
//     async fn build(self) -> Result<Self::Db, Self::Error> {
//         Ok(Db {
//             inner: self.inner.build().await?,
//         })
//     }
// }
//
// pub struct Db<StoreMarkers> {
//     inner: typesafe_idb::TypesafeDb<StoreMarkers>,
// }
//
// impl<StoreMarkers> storage::Db for Db<StoreMarkers>
// where
//     StoreMarkers: StoreMarker<User> + StoreMarker<Repository>,
// {
//     type Error = typesafe_idb::Error;
//
//     type TableMarkers = ConvertMarker<StoreMarkers>;
//
//     type TxnBuilder<'db, TxnTableMarkers, Mode>
//         = TxnBuilder<'db, Self::TableMarkers, ConvertMarker<TxnTableMarkers>, Mode>
//     where
//         Self: 'db,
//         StoreMarkers: 'db,
//         TxnTableMarkers: 'db,
//         Self::TableMarkers: 'db,
//         Mode: 'db;
//
//     fn txn_builder(&self) -> Self::TxnBuilder<'_, (), storage::ReadOnly> {
//         todo!()
//     }
// }
//
// pub struct TxnBuilder<'db, StoreMarkersFromDb, TxnStoreMarkers, Mode> {
//     inner: typesafe_idb::TxnBuilder<'db, StoreMarkersFromDb, TxnStoreMarkers, Mode>,
// }
//
// impl<'db, StoreMarkersFromDb, TxnStoreMarkers, Mode>
//     storage::TxnBuilder<
//         'db,
//         ConvertMarker<StoreMarkersFromDb>,
//         ConvertMarker<TxnStoreMarkers>,
//         Mode,
//     > for TxnBuilder<'db, ConvertMarker<StoreMarkersFromDb>, ConvertMarker<TxnStoreMarkers>, Mode>
// where
//     StoreMarkersFromDb: Default,
//     TxnStoreMarkers: Default,
// {
//     type Myself<S, T, M>
//         = TxnBuilder<'db, ConvertMarker<S>, ConvertMarker<T>, M>
//     where
//         S: 'db + Default,
//         T: 'db + Default,
//         M: 'db;
//     type Marker<U>
//         = ConvertMarker<U::Marker>
//     where
//         U: storage::Table;
//
//     type Txn;
//
//     fn build(self) -> Self::Txn {
//         todo!()
//     }
//
//     // fn read_only(
//     //     self,
//     // ) -> impl storage::TxnBuilder<
//     //     ConvertMarker<StoreMarkersFromDb>,
//     //     ConvertMarker<TxnStoreMarkers>,
//     //     storage::ReadOnly,
//     // > {
//     //     todo!()
//     // }
//     //
//     // fn read_write(
//     //     self,
//     // ) -> impl storage::TxnBuilder<
//     //     ConvertMarker<StoreMarkersFromDb>,
//     //     ConvertMarker<TxnStoreMarkers>,
//     //     storage::ReadWrite,
//     // > {
//     //     todo!()
//     // }
//     //
//     // fn with_table<U: storage::Table + typesafe_idb::Store + 'static>(
//     //     self,
//     // ) -> impl storage::TxnBuilder<
//     //     ConvertMarker<StoreMarkersFromDb>,
//     //     (
//     //         <U as typesafe_idb::Store>::Marker,
//     //         ConvertMarker<TxnStoreMarkers>,
//     //     ),
//     //     Mode,
//     // >
//     // where
//     //     ConvertMarker<StoreMarkersFromDb>: storage::TableMarkerFor<U>,
//     //     StoreMarkersFromDb: StoreMarker<U>,
//     // {
//     //     Self {
//     //         inner: self.inner.with_store::<U>(),
//     //     }
//     // }
// }
//
// trait Hi<T> {
//     type Myself<K>: Hi<K>;
//
//     fn returns_myself(&self) -> Self::Myself<T>;
// }
