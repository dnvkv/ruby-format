use std::path::PathBuf;

use super::severity::Severity;

pub struct Highlight {
    pub source_line: String,
    pub start: usize,
    pub lenght: usize,
}

pub struct Offense {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,

    pub severity: Severity,
    pub cop_name: String,
    pub message: String,

    pub highlight: Option<Highlight>,
}
