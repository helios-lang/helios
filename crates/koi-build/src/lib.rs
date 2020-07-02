use koi_driver::tokenize;
use koi_parser::reporter::{Diagnosis, Reporter};
use koi_parser::source::Source;

struct BuildReporter {
    // diagnosis_stack: Vec<Diagnosis>,
}

impl BuildReporter {
    pub fn new() -> Self {
        Self { /* diagnosis_stack: Vec::new() */ }
    }
}

impl Reporter for BuildReporter {
    fn report(&mut self, diagnosis: Diagnosis) {
        eprintln!(">>> ERROR: {:?}", diagnosis);
    }
}

/// Starts the build process with the given file.
pub fn build(file_name: &str) {
    let reporter = BuildReporter::new();

    match Source::file(file_name) {
        Ok(source) => {
            let time = std::time::Instant::now();
            tokenize(source, Box::new(reporter), false).iter().for_each(|token| println!("{:?}", token));
            println!("Time elapsed: {} ms", time.elapsed().as_millis());
        },
        Err(error) => {
            eprintln!("Failed to load file from source: {}", error);
        }
    }
}
