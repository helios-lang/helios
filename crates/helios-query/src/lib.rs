pub mod cancel;
pub mod input;
pub mod interner;
pub mod location;

pub use crate::input::*;
pub use crate::interner::*;
pub use crate::location::*;
use std::fmt::{self, Debug};

#[salsa::database(InputLocationDatabase, InputDatabase, InternerDatabase)]
#[derive(Default)]
pub struct HeliosDatabase {
    storage: salsa::Storage<HeliosDatabase>,
}

impl salsa::Database for HeliosDatabase {}

impl salsa::ParallelDatabase for HeliosDatabase {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(HeliosDatabase {
            storage: self.storage.snapshot(),
        })
    }
}

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
    fn test_source_location_queries() {
        let mut db = HeliosDatabase::default();
        db.set_source(FILE_ID, Arc::new(SOURCE.to_string()));

        assert_eq!(db.source(FILE_ID), Arc::new(SOURCE.to_string()));
        assert_eq!(db.source_len(FILE_ID), 31);

        let indexes = vec![0, 10, 20, 21, 31];
        assert_eq!(db.source_line_indexes(FILE_ID), Arc::new(indexes));

        assert_eq!(db.source_line_start(FILE_ID, 0), 0);
        assert_eq!(db.source_line_start(FILE_ID, 1), 10);
        assert_eq!(db.source_line_start(FILE_ID, 2), 20);
        assert_eq!(db.source_line_start(FILE_ID, 3), 21);
        assert_eq!(db.source_line_start(FILE_ID, 4), 31);

        assert_eq!(db.source_line_range(FILE_ID, 0), 0..10);
        assert_eq!(db.source_line_range(FILE_ID, 1), 10..20);
        assert_eq!(db.source_line_range(FILE_ID, 2), 20..21);
        assert_eq!(db.source_line_range(FILE_ID, 3), 21..31);
        assert_eq!(db.source_line_range(FILE_ID, 4), 31..31);

        assert_eq!(db.source_line_index(FILE_ID, 0), 0);
        assert_eq!(db.source_line_index(FILE_ID, 1), 0);
        assert_eq!(db.source_line_index(FILE_ID, 5), 0);
        assert_eq!(db.source_line_index(FILE_ID, 9), 0);
        assert_eq!(db.source_line_index(FILE_ID, 10), 1);
        assert_eq!(db.source_line_index(FILE_ID, 11), 1);
        assert_eq!(db.source_line_index(FILE_ID, 15), 1);
        assert_eq!(db.source_line_index(FILE_ID, 19), 1);
        assert_eq!(db.source_line_index(FILE_ID, 20), 2);
        assert_eq!(db.source_line_index(FILE_ID, 21), 3);
        assert_eq!(db.source_line_index(FILE_ID, 22), 3);
        assert_eq!(db.source_line_index(FILE_ID, 26), 3);
        assert_eq!(db.source_line_index(FILE_ID, 30), 3);
        assert_eq!(db.source_line_index(FILE_ID, 31), 4);

        assert_eq!(db.source_column_index(FILE_ID, 0, 0), 0);
        assert_eq!(db.source_column_index(FILE_ID, 0, 1), 1);
        assert_eq!(db.source_column_index(FILE_ID, 0, 5), 5);
        assert_eq!(db.source_column_index(FILE_ID, 0, 9), 9);
        assert_eq!(db.source_column_index(FILE_ID, 1, 10), 0);
        assert_eq!(db.source_column_index(FILE_ID, 1, 11), 1);
        assert_eq!(db.source_column_index(FILE_ID, 1, 15), 5);
        assert_eq!(db.source_column_index(FILE_ID, 1, 19), 9);
        assert_eq!(db.source_column_index(FILE_ID, 2, 20), 0);
        assert_eq!(db.source_column_index(FILE_ID, 3, 21), 0);
        assert_eq!(db.source_column_index(FILE_ID, 3, 22), 1);
        assert_eq!(db.source_column_index(FILE_ID, 3, 26), 5);
        assert_eq!(db.source_column_index(FILE_ID, 3, 30), 9);
        assert_eq!(db.source_column_index(FILE_ID, 4, 31), 0);

        assert_eq!(db.source_position_at_offset(FILE_ID, 0), (0, 0));
        assert_eq!(db.source_position_at_offset(FILE_ID, 1), (0, 1));
        assert_eq!(db.source_position_at_offset(FILE_ID, 5), (0, 5));
        assert_eq!(db.source_position_at_offset(FILE_ID, 9), (0, 9));
        assert_eq!(db.source_position_at_offset(FILE_ID, 10), (1, 0));
        assert_eq!(db.source_position_at_offset(FILE_ID, 11), (1, 1));
        assert_eq!(db.source_position_at_offset(FILE_ID, 15), (1, 5));
        assert_eq!(db.source_position_at_offset(FILE_ID, 19), (1, 9));
        assert_eq!(db.source_position_at_offset(FILE_ID, 20), (2, 0));
        assert_eq!(db.source_position_at_offset(FILE_ID, 21), (3, 0));
        assert_eq!(db.source_position_at_offset(FILE_ID, 22), (3, 1));
        assert_eq!(db.source_position_at_offset(FILE_ID, 26), (3, 5));
        assert_eq!(db.source_position_at_offset(FILE_ID, 30), (3, 9));
        assert_eq!(db.source_position_at_offset(FILE_ID, 31), (4, 0));
    }

    /*
    #[test]
    fn test_all_bindings() {
        fn print_bindings(
            db: &mut HeliosDatabase,
            bindings: Arc<Vec<BindingId>>,
        ) {
            for binding in bindings.iter() {
                let binding_data: BindingData =
                    db.lookup_intern_binding(*binding);
                println!("{:?} => {:?}", binding, binding_data.identifier);
            }
        }

        let mut db = HeliosDatabase::default();
        db.set_source(FILE_ID, Arc::new(SOURCE.to_string()));

        let bindings = db.all_bindings(FILE_ID);
        print_bindings(&mut db, bindings);
    }
    */
}
