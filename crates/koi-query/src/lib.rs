mod cancel;
mod input;

use crate::input::InputStorage;
use salsa::Database as SalsaDatabase;

#[salsa::database(InputStorage)]
#[derive(Debug, Default)]
pub struct KoiDatabase {
    runtime: salsa::Runtime<KoiDatabase>,
}

impl SalsaDatabase for KoiDatabase {
    fn salsa_runtime(&self) -> &salsa::Runtime<Self> {
        &self.runtime
    }

    fn salsa_runtime_mut(&mut self) -> &mut salsa::Runtime<Self> {
        &mut self.runtime
    }
}

#[cfg(test)]
mod tests {
    use crate::input::Input;
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_source_text() {
        let mut db = KoiDatabase::default();
        let file_name = "foo.koi".to_string();

        let mut num = 1;
        let mut power = 1;

        while num <= 10_000 {
            db.set_source_text(file_name.clone(), Arc::new(num.to_string()));
            assert_eq!(db.source_length(file_name.clone()), power);

            num = num * 10;
            power += 1;
        }
    }

    #[test]
    fn test_line_offsets_with_line_feed() {
        let mut db = KoiDatabase::default();
        let source = "let a = 10\nlet b = foo(a)\n\nIO.println(a + b)";
        let _file_name = "foo.koi".to_string();
        macro_rules! file_name { () => { _file_name.clone() } };

        db.set_source_text(file_name!(), Arc::new(source.to_string()));
        assert_eq!(db.source_line_offsets(file_name!()), vec![0, 11, 26, 27, 44]);

        // let b = fo|o(a) <- {1,10}
        let (line, column) = (1, 10);
        assert_eq!(db.source_offset_at_position(file_name!(), line, column), 21);
    }

    #[test]
    fn test_line_offsets_with_carriage_return_line_feed() {
        let mut db = KoiDatabase::default();
        let source = "let a = 10\r\nlet b = foo(a)\r\n\r\nIO.println(a + b)\r\n";
        let _file_name = "foo.koi".to_string();
        macro_rules! file_name { () => { _file_name.clone() } };

        db.set_source_text(file_name!(), Arc::new(source.to_string()));
        assert_eq!(db.source_line_offsets(file_name!()), vec![0, 12, 28, 30, 49]);

        // let b = fo|o(a) <- {1,10}
        let (line, column) = (1, 10);
        assert_eq!(db.source_offset_at_position(file_name!(), line, column), 22);
    }
}
