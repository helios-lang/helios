use koi_driver::tokenize;
use koi_syntax::source::Source;

/// Starts the build process with the given file.
pub fn build(file_name: &str) {
    match Source::file(file_name) {
        Ok(source) => {
            let time = std::time::Instant::now();
            tokenize(source).iter().for_each(|node| println!("{:?}", node));
            println!("Time elapsed: {} ms", time.elapsed().as_millis());
        },
        Err(error) => {
            eprintln!("Failed to load file from source: {}", error);
        }
    }
}
