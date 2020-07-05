#[macro_export]
/// Creates a new simple lexer test.
///
/// Requires a `str` literal (representing the source code) and a `Vec<Token>`
/// (representing the final tokenized form). This macro will tokenize the `str`
/// literal and its result will be determined if it is equal to the expected
/// tokens.
macro_rules! create_lexer_test {
    ($string:expr, $expected:expr) => {
        let string = $string;
        let mut s = read_from_string(string);
        let source = $crate::source::Source::stream(&mut s);
        match source {
            Ok(source) => {
                let mut tokens = Vec::new();
                let mut lexer = $crate::lexer::Lexer::with(source);

                while !lexer.is_at_end() {
                    tokens.push(lexer.next_token());
                }

                assert_eq!(tokens, $expected)
            },
            Err(error) => panic!("Failed to create Source from stream: {}", error)
        }
    };
}
