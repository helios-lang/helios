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
    use std::sync::Arc;
    use crate::input::Input;
    use super::*;

    #[test]
    fn test_source_text() {
        let mut db = KoiDatabase::default();
        let file_name = "foo.koi".to_string();

        let mut num = 1;
        let mut power = 1;

        while num <= 10_000 {
            db.set_source_text(file_name.clone(), Arc::new(format!("{}", num)));
            assert_eq!(db.source_length(file_name.clone()), power);

            num = num * 10;
            power += 1;
        }
    }

    #[test]
    fn test_line_offsets_with_line_feed() {
        let mut db = KoiDatabase::default();
        let file_name = "foo.koi".to_string();
        let source = "let a = 10\nlet b = foo(a)\n\nIO.println(a + b)\n";
        db.set_source_text(file_name.clone(), Arc::new(source.to_string()));

        assert_eq!(db.line_offsets(file_name), vec![0, 11, 26, 27, 45]);
    }

    #[test]
    fn test_line_offsets_with_carriage_return_line_feed() {
        let mut db = KoiDatabase::default();
        let file_name = "foo.koi".to_string();
        let source = "let a = 10\r\nlet b = foo(a)\r\n\r\nIO.println(a + b)\r\n";
        db.set_source_text(file_name.clone(), Arc::new(source.to_string()));

        assert_eq!(db.line_offsets(file_name), vec![0, 12, 28, 30, 49]);
    }
}
