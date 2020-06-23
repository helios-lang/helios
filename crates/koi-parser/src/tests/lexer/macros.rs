#[macro_export]
/// Creates a new simple test.
///
/// Requires a `str` literal (representing the source code) and a `Vec` of
/// `Token`s (representing the final tokenized form). The provided `str` literal
/// will be tokenized and assert if the tokenized form is equal to the given
/// `Vec`.
macro_rules! create_test {
    ($string:expr, $expected:expr) => {
        let string = $string;
        let mut s = read_from_string(string);
        let source = $crate::Source::stream(&mut s);
        match source {
            Ok(source) => {
                let mut tokens = Vec::new();
                let mut lexer = Lexer::with(source, true);

                while let Some(token) = lexer.next_token() {
                    tokens.push(token);
                }

                assert_eq!(tokens, $expected)
            },
            Err(error) => panic!("Failed to create Source from stream: {}", error)
        }
    };
}
