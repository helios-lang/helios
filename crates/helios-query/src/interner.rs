#[salsa::query_group(InternerDatabase)]
pub trait Interner: salsa::Database {
    #[salsa::interned]
    fn intern_binding(&self, binding: BindingData) -> BindingId;

    #[salsa::interned]
    fn intern_file(&self, file: FileData) -> FileId;
}

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

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct FileData {
    pub path: String,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct FileId(salsa::InternId);

impl salsa::InternKey for FileId {
    fn from_intern_id(id: salsa::InternId) -> Self {
        Self(id)
    }

    fn as_intern_id(&self) -> salsa::InternId {
        self.0
    }
}
