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

#[macro_export]
macro_rules! create_string_test {
    ($string:expr, $test_macro:ident, $expected:expr) => {
        create_string_test!($string, $test_macro, $expected, $string.len());
    };
    ($string:expr, $test_macro:ident, $expected:expr, $size:expr) => {
        let string = $string;
        let mut s = read_from_string(string);
        let source = $crate::Source::stream(&mut s);
        $test_macro!(source, $expected, $size);
    };
}

#[macro_export]
macro_rules! create_numeric_test {
    ($number:expr, $test_macro:ident, $expected:expr) => {
        create_numeric_test!($number, $test_macro, $expected, NumericBase::Decimal);
    };
    ($number:expr, $test_macro:ident, $expected:expr, $base:expr) => {
        let number = $number;
        let mut s = read_from_string(number);
        let source = Source::stream(&mut s);
        $test_macro!(source, $expected, $base, number.len())
    };
}
