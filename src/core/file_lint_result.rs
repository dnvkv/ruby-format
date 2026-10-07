use crate::core::{offense::Offense, severity::Severity};
use std::path::PathBuf;

pub struct FileLintResult {
    pub path: PathBuf,
    pub offences: Vec<Offense>,
}

impl FileLintResult {
    pub fn empty(path: PathBuf) -> Self {
        FileLintResult {
            path,
            offences: Vec::new(),
        }
    }

    pub fn new(path: PathBuf, offences: Vec<Offense>) -> Self {
        FileLintResult {
            path,
            offences: offences,
        }
    }

    pub fn severity(&self) -> Option<Severity> {
        self.offences.iter().map(|o| o.severity).max()
    }
}
