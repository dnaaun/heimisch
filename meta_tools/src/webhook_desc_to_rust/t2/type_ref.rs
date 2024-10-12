use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone, Eq)]
pub struct TypeRef<T> {
    r#type: T,
    name: String,
}

impl<T> TypeRef<T> {
    pub fn r#type(&self) -> &T {
        &self.r#type
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

/// The hash of this will depend only on `r#type` and not name to make it easier for me to
/// deduplicate only by structure, and not by name.
impl<T: Hash + Eq> Hash for TypeRef<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.r#type.hash(state)
    }
}

impl<T: Eq> PartialEq for TypeRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.r#type.eq(&other.r#type)
    }
}

impl<T: Eq + std::hash::Hash + Clone> TypeRef<T> {
    pub fn new_or_incr_count(
        r#type: T,
        name: String,
        type_ref_store: &mut HashMap<TypeRef<T>, u32>,
    ) -> Self {
        let type_ref = TypeRef { r#type, name };
        *type_ref_store.entry(type_ref.clone()).or_insert(0) += 1;
        type_ref
    }
}
