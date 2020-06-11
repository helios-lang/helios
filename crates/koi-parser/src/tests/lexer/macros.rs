#[macro_export]
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
                        Some(token) => tokens.push(token),
                        None => break,
                    }
                }

                assert_eq!(tokens, $expected)
            },
            Err(error) => panic!("Failed to create Source from stream: {}", error)
        }
    };
}
