use std::collections::BTreeMap;

pub struct NonEmpty2dMap<K1, K2, V> {
    inner: BTreeMap<K1, BTreeMap<K2, V>>,
}

impl<K1, K2, V> Default for NonEmpty2dMap<K1, K2, V> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<K1, K2, V> NonEmpty2dMap<K1, K2, V>
where
    K1: Ord,
    K2: Ord,
{
    pub fn insert(&mut self, k1: K1, k2: K2, v: V) -> Option<V> {
        self.inner.entry(k1).or_default().insert(k2, v)
    }

    pub fn remove(&mut self, k1: &K1, k2: &K2) -> Option<V> {
        if self.inner.contains_key(k1) {
            let v = self.inner.get_mut(k1).expect("").remove(k2);
            if self.inner.get(k1).expect("").len() == 0 {
                self.inner.remove(k1);
            }
            v
        } else {
            None
        }
    }

    pub fn get(&mut self, k1: &K1, k2: &K2) -> Option<&V> {
        self.inner.get(k1).map(|deep| deep.get(k2)).flatten()
    }

    pub fn get_mut(&mut self, k1: &K1, k2: &K2) -> Option<&mut V> {
        self.inner
            .get_mut(k1)
            .map(|deep| deep.get_mut(k2))
            .flatten()
    }
}
