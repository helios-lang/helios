use koi_driver::parse;
use koi_syntax::source::Source;

/// Starts the build process with the given file.
pub fn build(file_name: &str) {
    match Source::file(file_name) {
        Ok(source) => {
            let time = std::time::Instant::now();
            parse(source); //.nodes().iter().for_each(|node| eprintln!("{:#?}", node));
            println!("Time elapsed: {} ms", time.elapsed().as_millis());
        },
        Err(error) => {
            eprintln!("Failed to load file from source: {}", error);
        }
    }
}
