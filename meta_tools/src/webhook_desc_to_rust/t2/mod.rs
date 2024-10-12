mod postprocess;
mod type_ref;

use itertools::Itertools;
use ordered_float::NotNan;
pub use postprocess::postprocess_t2_inner;
use std::collections::HashSet;
use std::hash::Hash;
pub use type_ref::TypeRef;

#[derive(Debug, Clone)]
pub struct Type {
    pub inner: TypeInner,
    pub meta: Meta,
}

impl Type {
    pub fn map_inner(self, mut func: impl FnMut(TypeInner) -> TypeInner) -> Self {
        let Self { inner, meta } = self;
        Self {
            inner: func(inner),
            meta,
        }
    }
}

/// Hashing is used to deduplicate types, and I don't want that deduplication to depend on
/// metadata.
impl Hash for Type {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}
impl Eq for Type {}

#[derive(Debug, Clone)]
pub struct Meta {
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_required: bool,
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            title: None,
            description: None,
            is_required: false,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TypeInner {
    Unspecified,
    Null,
    String(StringInner),
    Integer(IntegerInner),
    Float(FloatInner),
    Boolean(BooleanInner),
    Nullable(Box<TypeInner>),
    Union(UnionInner),
    Array(ArrayInner),

    LiteralBool(bool),
    LiteralString(String),

    LiteralStringUnionRef(TypeRef<LiteralStringUnionInner>),
    ObjectRef(TypeRef<ObjectInner>),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ObjectInner {
    pub members: Vec<ObjectMember>,
}

impl ObjectMember {
    pub fn map_value(self, mut func: impl FnMut(Type) -> Type) -> Self {
        let ObjectMember { key, value } = self;
        ObjectMember {
            key,
            value: func(value),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ObjectMember {
    pub key: String,
    pub value: Type,
}

impl From<TypeRef<ObjectInner>> for TypeInner {
    fn from(value: TypeRef<ObjectInner>) -> Self {
        TypeInner::ObjectRef(value)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ArrayInner {
    pub members: Box<TypeInner>,
}

impl From<ArrayInner> for TypeInner {
    fn from(value: ArrayInner) -> Self {
        TypeInner::Array(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralStringUnionInner {
    pub members: HashSet<String>,
    pub default: Option<String>,
}

impl Hash for LiteralStringUnionInner {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Sorting is necessary to ensure the hashing returns the same value across multipel
        // invocations.
        self.members.iter().sorted().collect_vec().hash(state);
        self.default.hash(state);
    }
}

impl From<TypeRef<LiteralStringUnionInner>> for TypeInner {
    fn from(value: TypeRef<LiteralStringUnionInner>) -> Self {
        TypeInner::LiteralStringUnionRef(value)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct UnionInner {
    pub variants: Vec<Box<TypeInner>>,
}

impl From<UnionInner> for TypeInner {
    fn from(value: UnionInner) -> Self {
        TypeInner::Union(value)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct BooleanInner {
    pub default: Option<bool>,
}

impl From<BooleanInner> for TypeInner {
    fn from(value: BooleanInner) -> Self {
        TypeInner::Boolean(value)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub struct IntegerInner {
    pub default: Option<i64>,
}

impl From<IntegerInner> for TypeInner {
    fn from(value: IntegerInner) -> Self {
        TypeInner::Integer(value)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FloatInner {
    pub default: Option<NotNan<f64>>,
}

impl From<FloatInner> for TypeInner {
    fn from(value: FloatInner) -> Self {
        TypeInner::Float(value)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub struct StringInner {
    pub default: Option<String>,
}

impl From<StringInner> for TypeInner {
    fn from(value: StringInner) -> Self {
        TypeInner::String(value)
    }
}
