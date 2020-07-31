#[test]
fn test() {
    use crate::source::*;
    match Source::file("/Users/taseen/Documents/Programming/Koi/tests/stage 2/Simple.koi") {
        Ok(source) => {
            let mut cursor = Cursor::with(source);
            while let Some(c) = cursor.advance() {
                print!("{}", c);
            }
        },
        Err(error) => {
            panic!("Failed to read from source: {}", error)
        }
    }
}
