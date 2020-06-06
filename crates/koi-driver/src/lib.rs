mod diagnostic;

pub use diagnostic::Diagnostic;

type Result<T> = std::result::Result<T, Vec<Diagnostic>>;

pub fn start(file_name: &str) -> Result<()> {
    println!("file_name: {}", file_name);
    Err(vec![Diagnostic::new("E0001", "Error message")])
}
