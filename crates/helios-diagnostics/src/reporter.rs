use crate::Diagnostic;

pub struct DiagnosticReporter {
    pub diagnostics: Vec<Diagnostic>,
}

impl DiagnosticReporter {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub fn report(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic)
    }

    pub fn clear(&mut self) {
        self.diagnostics.clear()
    }
}
