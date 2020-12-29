mod cancel;
mod input;
mod interner;

use crate::input::*;
use crate::interner::*;
use std::fmt::{self, Debug};

#[salsa::database(InputDatabase, InternerDatabase)]
#[derive(Default)]
pub struct HeliosDatabase {
    storage: salsa::Storage<HeliosDatabase>,
}

impl salsa::Database for HeliosDatabase {}

impl Debug for HeliosDatabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HeliosDatabase").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    const FILE_ID: usize = 0;
    const SOURCE: &str = "let a = 0\nlet b = 1\n\nlet c = 2\n";

    #[test]
    fn test_source_length() {
        let mut db = HeliosDatabase::default();
        db.set_source(FILE_ID, Arc::new(SOURCE.to_string()));

        assert_eq!(db.source_len(FILE_ID), 31);

        let indexes = vec![0, 10, 20, 21, 31];
        assert_eq!(db.source_line_indexes(FILE_ID), Arc::new(indexes));
    }

    #[test]
    fn test_all_bindings() {
        fn print_bindings(
            db: &mut HeliosDatabase,
            bindings: Arc<Vec<BindingId>>,
        ) {
            for binding in bindings.iter() {
                let binding_data = db.lookup_intern_binding(*binding);
                println!("{:?} => {}", binding, binding_data.identifier);
            }
        }

        let mut db = HeliosDatabase::default();
        db.set_source(FILE_ID, Arc::new(SOURCE.to_string()));

        let bindings = db.all_bindings(FILE_ID);
        print_bindings(&mut db, bindings);
    }
}
