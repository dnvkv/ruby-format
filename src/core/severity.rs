#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Refactor,
    Convention,
    Warning,
    Error,
    Fatal,
}
