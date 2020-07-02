use crate::source::Position;
use std::ops::Range;

#[derive(Debug)]
pub enum Location {
    Position(Position),
    Span(Position, Position),
}

#[derive(Debug)]
pub struct Diagnosis {
    message: String,
    location: Location,
}

impl Diagnosis {
    pub fn new<S: Into<String>>(message: S, location: Location) -> Self {
        Self { message: message.into(), location }
    }
}

pub trait Reporter {
    fn report(&mut self, diagnosis: Diagnosis);
}
