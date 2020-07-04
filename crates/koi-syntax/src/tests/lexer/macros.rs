#[macro_export]
/// Creates a new simple test.
///
/// Requires a `str` literal (representing the source code) and a `Vec<Token>`
/// (representing the expected tokenized form). This macro will then tokenize
/// the source code, after which its result will be checked with the expected
/// form.
macro_rules! create_test {
    ($string:expr, $expected:expr) => {
        let string = $string;
        let mut s = read_from_string(string);
        let source = $crate::Source::stream(&mut s);
        match source {
            Ok(source) => {
                let mut tokens = Vec::new();
                let mut lexer = Lexer::with(source);

                loop {
                    match lexer.next_token() {
                        token if token.kind == TokenKind::Eof => break,
                        token => tokens.push(token),
                    }
                }

                assert_eq!(tokens, $expected)
            },
            Err(error) => panic!("Failed to create Source from stream: {}", error)
        }
    };
}
