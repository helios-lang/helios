#[macro_export]
macro_rules! create_test {
    ($string:expr, $test_macro:ident, $expected:expr) => {
        create_test!($string, $test_macro, $expected, $string.len());
    };
    ($string:expr, $test_macro:ident, $expected:expr, $size:expr) => {
        let string = $string;
        let mut s = read_from_string(string);
        let s = $crate::Source::stream(&mut s);
        $test_macro!(s, $expected, $size);
    };
}
