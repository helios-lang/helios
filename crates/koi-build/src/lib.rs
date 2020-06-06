use koi_driver::Diagnostic;
use std::io::Write;

pub fn emit_diagnostic<I, W>(writer: &mut W, diagnostics: I)
where I: IntoIterator<Item=Diagnostic>,
      W: Write
{
    for diagnostic in diagnostics {
        writeln!(writer, "[{}]: {}", diagnostic.code, diagnostic.message)
            .expect("Failed to write to writer");
    }
}

/// Starts the build process with the given file.
pub fn build(file_name: &str) {
    if let Err(diagnostics) = koi_driver::start(file_name) {
        emit_diagnostic(&mut std::io::stderr().lock(), diagnostics);
    }
}
