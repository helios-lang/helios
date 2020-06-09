#[macro_use] mod macros;
mod keywords;
mod literals;

fn read_from_string(s: &str) -> &[u8] {
    s.as_bytes()
}
