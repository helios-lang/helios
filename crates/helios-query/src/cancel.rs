use std::error::Error;
use std::fmt::{self, Display};

#[allow(dead_code)]
pub type Cancelable<T> = Result<T, Cancelled>;

#[derive(Debug)]
pub struct Cancelled;

impl Display for Cancelled {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cancelled")
    }
}

impl Error for Cancelled {}
