#[macro_use] mod macros;
mod expressions;
mod keywords;
mod literals;
mod symbols;

fn read_from_string(s: &str) -> &[u8] {
    s.as_bytes()
}
