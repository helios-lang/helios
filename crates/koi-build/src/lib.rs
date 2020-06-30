use koi_driver::tokenize;
use koi_parser::source::Source;

/// Starts the build process with the given file.
pub fn build(file_name: &str) {
    match Source::file(file_name) {
        Ok(source) => {
            let time = std::time::Instant::now();
            tokenize(source, false).iter().for_each(|token| println!("{:?}", token));
            println!("Time elapsed: {} ms", time.elapsed().as_millis());
        },
        Err(error) => {
            eprintln!("Failed to load file from source: {}", error);
        }
    }
}
