mod cancel;
mod input;

use crate::input::InputStorage;
use salsa::Database as SalsaDatabase;

#[salsa::database(InputStorage)]
#[derive(Debug, Default)]
pub struct HeliosDatabase {
    runtime: salsa::Runtime<HeliosDatabase>,
}

impl SalsaDatabase for HeliosDatabase {
    fn salsa_runtime(&self) -> &salsa::Runtime<Self> {
        &self.runtime
    }

    fn salsa_runtime_mut(&mut self) -> &mut salsa::Runtime<Self> {
        &mut self.runtime
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::Input;
    use std::sync::Arc;

    #[test]
    #[allow(non_snake_case)]
    fn test_source_locations_with_LF() {
        let mut db = HeliosDatabase::default();
        let _file_name = "foo.he".to_string();
        macro_rules! file_name {
            () => {
                _file_name.clone()
            };
        };

        let contents = "let a = 10\nlet b = foo(a)\n\nIO.println(a + b)\n";
        db.set_source_text(file_name!(), Arc::new(contents.to_string()));
        assert_eq!(
            db.source_line_offsets(file_name!()),
            vec![0, 11, 26, 27, 45]
        );

        assert_eq!(db.source_offset_at_position(file_name!(), 1, 10), 21);
        assert_eq!(db.source_offset_at_position(file_name!(), 2, 0), 26);
        assert_eq!(db.source_offset_at_position(file_name!(), 3, 8), 35);

        assert_eq!(db.source_position_at_offset(file_name!(), 21), (1, 10));
        assert_eq!(db.source_position_at_offset(file_name!(), 26), (2, 0));
        assert_eq!(db.source_position_at_offset(file_name!(), 35), (3, 8));
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_source_locations_with_CRLF() {
        let mut db = HeliosDatabase::default();
        let _file_name = "foo.he".to_string();
        macro_rules! file_name {
            () => {
                _file_name.clone()
            };
        };

        let contents =
            "let a = 10\r\nlet b = foo(a)\r\n\r\nIO.println(a + b)\r\n";
        db.set_source_text(file_name!(), Arc::new(contents.to_string()));
        assert_eq!(
            db.source_line_offsets(file_name!()),
            vec![0, 12, 28, 30, 49]
        );

        assert_eq!(db.source_offset_at_position(file_name!(), 1, 10), 22);
        assert_eq!(db.source_offset_at_position(file_name!(), 2, 0), 28);
        assert_eq!(db.source_offset_at_position(file_name!(), 3, 8), 38);

        assert_eq!(db.source_position_at_offset(file_name!(), 22), (1, 10));
        assert_eq!(db.source_position_at_offset(file_name!(), 28), (2, 0));
        assert_eq!(db.source_position_at_offset(file_name!(), 38), (3, 8));
    }
}
