#[test]
fn test() {
    use crate::source::*;
    let path = "/Users/taseen/Documents/Programming/Koi/tests/stage 2/Simple.koi";
    let source = std::fs::read_to_string(path).expect("Failed to read file");
    let mut cursor = Cursor::with(source);

    while let Some(c) = cursor.advance() {
        print!("{}", c);
    }
}
