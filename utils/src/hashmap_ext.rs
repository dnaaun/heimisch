use std::{collections::HashMap, hash::Hash};

pub trait HashMapExt<K, V> {
    fn map_values<'a, R>(&'a self, func: impl Fn(&'a V) -> R) -> HashMap<K, R>
    where
        K: Clone,
        V: 'a;
    fn into_map_values<R>(self, func: impl Fn(V) -> R) -> HashMap<K, R>;
}

impl<K, V> HashMapExt<K, V> for HashMap<K, V>
where
    K: Hash,
    K: Eq,
{
    fn map_values<'a, R>(&'a self, func: impl Fn(&'a V) -> R) -> HashMap<K, R>
    where
        K: Clone,
        V: 'a,
    {
        self.iter().map(|(k, v)| (k.clone(), func(v))).collect()
    }

    fn into_map_values<R>(self, mut func: impl FnMut(V) -> R) -> HashMap<K, R> {
        self.into_iter().map(|(k, v)| (k, func(v))).collect()
    }
}
