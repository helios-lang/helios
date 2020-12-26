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

    #[test]
    #[allow(non_snake_case)]
    fn test_source_locations_with_LF() {
        const FILE_ID: usize = 0;
        let mut db = HeliosDatabase::default();

        let contents =
            "let a = 10 in\nlet b = foo a in\n\nIO.println (a + b)\n";
        db.set_source_text(FILE_ID, contents.to_string());

        let line_offsets = vec![0, 14, 31, 32, 51];
        assert_eq!(db.source_line_offsets(FILE_ID), line_offsets);

        assert_eq!(db.source_offset_at_position(FILE_ID, 1, 10), 24);
        assert_eq!(db.source_offset_at_position(FILE_ID, 2, 0), 31);
        assert_eq!(db.source_offset_at_position(FILE_ID, 3, 8), 40);

        assert_eq!(db.source_position_at_offset(FILE_ID, 24), (1, 10));
        assert_eq!(db.source_position_at_offset(FILE_ID, 31), (2, 0));
        assert_eq!(db.source_position_at_offset(FILE_ID, 40), (3, 8));
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_source_locations_with_CRLF() {
        const FILE_ID: usize = 0;
        let mut db = HeliosDatabase::default();

        let contents =
            "let a = 10 in\r\nlet b = foo a in\r\n\r\nIO.println (a + b)\r\n";
        db.set_source_text(FILE_ID, contents.to_string());

        let line_offsets = vec![0, 15, 33, 35, 55];
        assert_eq!(db.source_line_offsets(FILE_ID), line_offsets);

        assert_eq!(db.source_offset_at_position(FILE_ID, 1, 10), 25);
        assert_eq!(db.source_offset_at_position(FILE_ID, 2, 0), 33);
        assert_eq!(db.source_offset_at_position(FILE_ID, 3, 8), 43);

        assert_eq!(db.source_position_at_offset(FILE_ID, 25), (1, 10));
        assert_eq!(db.source_position_at_offset(FILE_ID, 33), (2, 0));
        assert_eq!(db.source_position_at_offset(FILE_ID, 43), (3, 8));
    }
}
