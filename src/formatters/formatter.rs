use crate::core::file_lint_result::FileLintResult;

pub trait Formatter {
    fn started(&mut self, file_count: usize);
    fn file_finished(&mut self, result: &FileLintResult);
    fn finished(&mut self, results: &Vec<FileLintResult>);
}
