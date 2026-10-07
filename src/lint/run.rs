use std::path::PathBuf;

use ruby_prism::{ClassNode, Visit, parse};

use super::parse::ParseError;
use crate::core::file_lint_result::FileLintResult;
use crate::core::offense::{Highlight, Offense};
use crate::core::severity::Severity;
use crate::formatters::formatter::Formatter;

pub struct Linter {
    formatter: Box<dyn Formatter>,
}

pub struct LinterRunResult {
    linter_results: Vec<FileLintResult>,
    failed: Vec<FileLintFailed>,
}

pub struct FileLintFailed {
    path: PathBuf,
    error: std::io::Error,
}

impl Linter {
    pub fn new(formatter: Box<dyn Formatter>) -> Self {
        Linter { formatter }
    }

    pub fn lint_all(&mut self, files: Vec<PathBuf>) -> LinterRunResult {
        let mut result = LinterRunResult {
            linter_results: Vec::new(),
            failed: Vec::new(),
        };
        self.formatter.started(files.len());
        for target_file in files {
            match std::fs::read(&target_file) {
                Ok(file_contents) => {
                    let file_result = self.lint_file(target_file, file_contents);
                    result.linter_results.push(file_result)
                }
                Err(error) => result.failed.push(FileLintFailed {
                    path: target_file,
                    error,
                }),
            }
        }
        self.formatter.finished(&result.linter_results);
        result
    }

    fn lint_file(&mut self, path: PathBuf, content: Vec<u8>) -> FileLintResult {
        let parse_result = parse(&content);
        let mut parse_errors: Vec<ParseError> = Vec::new();

        for parse_error in parse_result.errors() {
            parse_errors.push(ParseError::new(
                parse_error.location().start_offset(),
                parse_error.location().end_offset(),
                &content,
                parse_error.message().to_string(),
            ))
        }

        if parse_errors.is_empty() {
            let mut visitor = DefinitionsAnalyzer;
            visitor.visit(&parse_result.node());

            let result = FileLintResult::empty(path);
            self.formatter.file_finished(&result);
            result
        } else {
            let offenses = parse_errors
                .into_iter()
                .map(|parse_error| Offense {
                    file: path.clone(),
                    line: parse_error.diagnostics.line_number,
                    column: parse_error.diagnostics.col_number,
                    severity: Severity::Fatal,
                    cop_name: "Lint/Syntax".to_string(),
                    message: parse_error.message,
                    highlight: parse_error.diagnostics.line.map(|source_line| Highlight {
                        source_line,
                        start: parse_error.diagnostics.line_number,
                        lenght: 1,
                    }),
                })
                .collect();
            let result = FileLintResult::new(path, offenses);
            self.formatter.file_finished(&result);
            result
        }
    }
}

struct DefinitionsAnalyzer;

impl<'a> Visit<'a> for DefinitionsAnalyzer {
    fn visit_class_node(&mut self, node: &ClassNode<'a>) {
        // match std::str::from_utf8(node.name().as_slice()) {
        //     Ok(name) => println!("Found a class {}", name),
        //     Err(_) => (),
        // };
        ruby_prism::visit_class_node(self, node);
    }
}
