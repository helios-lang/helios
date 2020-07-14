use koi_driver::parse;
use koi_syntax_old::source::Source;

/// Starts the build process with the given file.
pub fn build(file_name: &str) {
    match Source::file(file_name) {
        Ok(source) => {
            let time = std::time::Instant::now();
            let ast = parse(source);
            println!("{:#?}", ast.nodes());
            println!("Program span: {}", ast.span());
            println!("Time elapsed: {} ms", time.elapsed().as_millis());
        },
        Err(error) => {
            eprintln!("Failed to load file from source: {}", error);
        }
    }
}
