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

#[macro_export]
macro_rules! create_numeric_test {
    ($number:expr, $test_macro:ident, $expected:expr) => {
        create_numeric_test!($number, $test_macro, $expected, NumericBase::Decimal);
    };
    ($number:expr, $test_macro:ident, $expected:expr, $base:expr) => {
        let number = $number;
        let mut s = read_from_string(number);
        let s = Source::stream(&mut s);
        $test_macro!(s, $expected, $base, number.len())
    };
}
