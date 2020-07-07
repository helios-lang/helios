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
            assert_eq!(db.length(file_name.clone()), power);

            num = num * 10;
            power += 1;
        }
    }

    #[test]
    fn test_ast() {
        let mut db = KoiDatabase::default();
        let file_name = "foo.koi".to_string();
        db.set_source_text(file_name.clone(), Arc::new("10".to_string()));

        let mut step = 1;
        let mut lines = 1;
        while lines <= 50 {
            println!("{}", db.length(file_name.clone()));

            if step % 5 == 0 {
                db.set_source_text(
                    file_name.clone(),
                    Arc::new(
                        format!(
                            "{}{}",
                            db.source_text(file_name.clone()),
                            "\n10",
                        )
                    )
                );
                lines += 1;
            }

            step += 1;
        }
    }
}
