#[macro_use] mod macros;
mod expressions;

fn read_from_string(s: &str) -> &[u8] {
    s.as_bytes()
}
