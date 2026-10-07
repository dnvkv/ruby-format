use colored::{ColoredString, Colorize};

use super::formatter::Formatter;
use crate::core::file_lint_result::FileLintResult;
use crate::core::offense::Offense;
use crate::core::severity::Severity;

use std::io::Write;
use std::path::Path;

pub struct ProgressFormatter;

impl Formatter for ProgressFormatter {
    fn started(&mut self, file_count: usize) {
        println!("Inspecting {file_count} files");
    }

    fn file_finished(&mut self, result: &FileLintResult) {
        print!("{}", progress_char(result));

        std::io::stdout().flush().unwrap();
    }

    fn finished(&mut self, results: &Vec<FileLintResult>) {
        println!();

        let offending: Vec<&FileLintResult> = results
            .iter()
            .filter(|result| !result.offences.is_empty())
            .collect();

        if !offending.is_empty() {
            println!();
            println!("Offenses:");
            println!();

            for result in &offending {
                report_file(result);
            }
        }

        let offense_count = offending
            .iter()
            .map(|result| result.offences.len())
            .sum::<usize>();

        report_summary(results.len(), offense_count);
    }
}

fn progress_char(result: &FileLintResult) -> String {
    match result.severity() {
        None => ".".green(),
        Some(Severity::Info) => "I".truecolor(190, 190, 190),
        Some(Severity::Refactor) => "R".yellow(),
        Some(Severity::Convention) => "C".yellow(),
        Some(Severity::Warning) => "W".magenta(),
        Some(Severity::Error) => "E".red(),
        Some(Severity::Fatal) => "F".red(),
    }
    .to_string()
}

fn report_file(result: &FileLintResult) {
    for offense in &result.offences {
        report_offense(offense);
    }
}

fn report_offense(offense: &Offense) {
    println!(
        "{}:{}:{}: {}: {}",
        to_user_friendly_path(&offense.file).cyan(),
        offense.line,
        offense.column + 1,
        colored_severity_code(offense.severity),
        message(offense)
    );

    let Some(highlight) = &offense.highlight else {
        return;
    };

    if highlight.source_line.trim().is_empty() {
        return;
    }

    println!("{}", highlight.source_line);
    println!(
        "{}{}",
        to_whitespace(&highlight.source_line, highlight.start),
        "^".repeat(highlight.lenght)
    );
}

fn report_summary(file_count: usize, offense_count: usize) {
    let files = pluralize(file_count, "file", false);
    let offenses = pluralize(offense_count, "offense", true);
    let offenses = if offense_count == 0 {
        offenses.green()
    } else {
        offenses.red()
    };

    println!();
    println!("{files} inspected, {offenses} detected");
}

fn message(offense: &Offense) -> String {
    format!(
        "{}: {}",
        offense.cop_name,
        annotate_message(&offense.message)
    )
}

fn annotate_message(message: &str) -> String {
    let mut annotated = String::new();
    let mut rest = message;

    while let Some(opening) = rest.find('`') {
        let after_opening = &rest[opening + 1..];

        let Some(closing) = after_opening.find('`') else {
            break;
        };

        annotated.push_str(&rest[..opening]);
        annotated.push_str(&after_opening[..closing].yellow().to_string());
        rest = &after_opening[closing + 1..];
    }

    annotated.push_str(rest);
    annotated
}

fn colored_severity_code(severity: Severity) -> ColoredString {
    match severity {
        Severity::Info => "I".truecolor(190, 190, 190),
        Severity::Refactor => "R".yellow(),
        Severity::Convention => "C".yellow(),
        Severity::Warning => "W".magenta(),
        Severity::Error => "E".red(),
        Severity::Fatal => "F".red(),
    }
}

fn to_whitespace(source_line: &str, column: usize) -> String {
    let prefix = source_line.chars().take(column);

    let mut whitespace: String = prefix.clone().filter(|char| *char == '\t').collect();
    whitespace.extend(prefix.filter(|char| *char != '\t').map(|_| ' '));

    whitespace
}

fn pluralize(number: usize, thing: &str, no_for_zero: bool) -> String {
    match number {
        0 if no_for_zero => format!("no {thing}s"),
        1 => format!("1 {thing}"),
        _ => format!("{number} {thing}s"),
    }
}

fn to_user_friendly_path(path: &Path) -> String {
    std::env::current_dir()
        .ok()
        .and_then(|base_dir| path.strip_prefix(base_dir).ok())
        .unwrap_or(path)
        .display()
        .to_string()
}
