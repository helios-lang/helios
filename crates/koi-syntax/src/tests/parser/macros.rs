#[macro_export]
/// Creates a new simple parser test.
///
/// Requires a `str` literal (representing the source code) and a `Vec<Node>`
/// (representing the final AST). This macro will tokenize and parse the `str`
/// literal and its result will be determined if it is equal to the expected
/// AST.
macro_rules! create_parser_test {
    ($string:expr, $expected:expr) => {
        let mut s = super::read_from_string($string);
        let source = $crate::source::Source::stream(&mut s);
        match source {
            Ok(source) => assert_eq!(crate::parse(source), $expected),
            Err(error) => panic!("Failed to create Source from stream: {}", error)
        }
    };
}
