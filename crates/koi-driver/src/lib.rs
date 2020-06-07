mod diagnostic;

pub use diagnostic::Diagnostic;

type Result<T> = std::result::Result<T, Vec<Diagnostic>>;

#[allow(dead_code)]
pub struct Driver<'a> {
    file_name: &'a str,
}

impl<'a> Driver<'a> {
    pub fn with(file_name: &'a str) -> Self {
        Self { file_name }
    }

    pub fn start(self) -> Result<()> {
        self.tokenize_source()?;
        Ok(())
    }

    pub fn tokenize_source(self) -> Result<()> {
        Err(vec![Diagnostic::new("E0001", "Error message")])
    }
}

pub fn start(file_name: &str) -> Result<()> {
    // println!("file_name: {}", file_name);
    let driver = Driver::with(file_name);
    driver.start()
}
