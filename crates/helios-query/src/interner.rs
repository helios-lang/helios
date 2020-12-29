#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct BindingData {
    pub identifier: String,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct BindingId(salsa::InternId);

impl salsa::InternKey for BindingId {
    fn from_intern_id(id: salsa::InternId) -> Self {
        Self(id)
    }

    fn as_intern_id(&self) -> salsa::InternId {
        self.0
    }
}

#[salsa::query_group(InternerDatabase)]
pub trait Interner: salsa::Database {
    #[salsa::interned]
    fn intern_binding(&self, binding: BindingData) -> BindingId;
}
