use std::fs;
use std::process::ExitCode;

pub mod cli;
pub mod config;
pub mod core;
pub mod files;
pub mod formatters;
pub mod lint;

use clap::Parser;

use crate::cli::Cli;
use crate::config::config_reader::PlainReader;
use crate::files::TargetsDetector;
use crate::formatters::progress::ProgressFormatter;
use crate::lint::run::Linter;

fn main() -> ExitCode {
    // Parse command line arguments
    // Resolve configuration chain
    // Resolve all targets map
    // For each target file iterate in parallel:
    //   * read all eligible cops
    //   * if there no eligible cops, skip the file
    //   * split all eligible cops into line-based and ast-based
    //   * run linting-correcting loop
    let cli = Cli::parse();
    let config_reader = PlainReader::new();
    let targets_detector = TargetsDetector::new(config_reader).unwrap();
    let formatter = ProgressFormatter;
    let mut linter = Linter::new(Box::new(formatter));
    let targets = if cli.targets.is_empty() {
        std::env::current_dir()
            .ok()
            .map(|current_dir| vec![current_dir])
            .unwrap_or(Vec::new())
    } else {
        cli.targets
    };

    let targets = targets_detector.find_for_all(targets).unwrap();
    linter.lint_all(targets);

    return ExitCode::SUCCESS;
}
