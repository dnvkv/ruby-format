use std::error::Error;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;

use glob::{Pattern, PatternError};
use walkdir::{Error as WalkDirError, WalkDir};

use crate::config::config_reader::ConfigReader;

pub struct TargetsDetector {
    include_patterns: Vec<Pattern>,
    exclude_patterns: Vec<Pattern>,
}

impl TargetsDetector {
    pub fn new(config_reader: impl ConfigReader) -> Result<TargetsDetector, TargetsDetectorError> {
        let mut include_patterns = Vec::new();
        let mut exclude_patterns = Vec::new();
        for pattern in &config_reader.read_global().include {
            include_patterns.push(Pattern::new(&pattern)?)
        }
        for pattern in &config_reader.read_global().exclude {
            exclude_patterns.push(Pattern::new(&pattern)?)
        }
        Ok(TargetsDetector {
            include_patterns,
            exclude_patterns,
        })
    }

    pub fn find_for_all<T: AsRef<Path>>(
        &self,
        targets: Vec<T>,
    ) -> Result<Vec<PathBuf>, TargetsDetectorError> {
        let mut result = Vec::new();
        for target in targets {
            let resolved = self.find_for_target(target)?;
            result.extend(resolved);
        }
        result.dedup();
        Ok(result)
    }

    pub fn find_for_target<T: AsRef<Path>>(
        &self,
        target: T,
    ) -> Result<Vec<PathBuf>, TargetsDetectorError> {
        let mut result_files = Vec::new();
        if std::fs::exists(&target)? {
            if std::fs::metadata(&target)?.is_file() {
                if self.is_a_target(&target) {
                    result_files.push(target.as_ref().to_owned());
                };
                Ok(result_files)
            } else {
                let target_files = WalkDir::new(&target).into_iter().filter_map(|e| e.ok());
                for entry in target_files {
                    let metadata = entry.metadata()?;
                    if metadata.is_file() && self.is_a_target(entry.path()) {
                        result_files.push(entry.path().to_owned())
                    }
                }
                return Ok(result_files);
            }
        } else {
            Ok(result_files)
        }
    }

    // TODO: refactor with all / any
    fn is_a_target<T: AsRef<Path>>(&self, target: T) -> bool {
        for pattern in &self.exclude_patterns {
            if pattern.matches(&target.as_ref().to_string_lossy()) {
                return false;
            }
        }
        for pattern in &self.include_patterns {
            if pattern.matches(&target.as_ref().to_string_lossy()) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
pub enum TargetsDetectorError {
    PatternCompilationError(PatternError),
    IoError(std::io::Error),
    WalkingError(WalkDirError),
}

impl From<PatternError> for TargetsDetectorError {
    fn from(error: PatternError) -> Self {
        TargetsDetectorError::PatternCompilationError(error)
    }
}

impl From<std::io::Error> for TargetsDetectorError {
    fn from(error: std::io::Error) -> Self {
        TargetsDetectorError::IoError(error)
    }
}

impl From<WalkDirError> for TargetsDetectorError {
    fn from(error: WalkDirError) -> Self {
        TargetsDetectorError::WalkingError(error)
    }
}

impl fmt::Display for TargetsDetectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TargetsDetectorError::PatternCompilationError(pattern_error) => {
                write!(f, "Pattern compilation error: {}", pattern_error)
            }
            TargetsDetectorError::IoError(io_error) => {
                write!(f, "IO error: {}", io_error)
            }
            TargetsDetectorError::WalkingError(walkdir_error) => {
                write!(f, "Walking error: {}", walkdir_error)
            }
        }
    }
}

impl Error for TargetsDetectorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TargetsDetectorError::PatternCompilationError(pattern_error) => Some(pattern_error),
            TargetsDetectorError::IoError(io_error) => Some(io_error),
            TargetsDetectorError::WalkingError(walkdir_error) => Some(walkdir_error),
        }
    }
}
