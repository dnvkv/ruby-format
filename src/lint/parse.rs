use std::string::FromUtf8Error;

pub struct ParseError {
    pub start_offset: usize,
    pub end_offset: usize,
    pub message: String,
    pub diagnostics: DiagnosticsLocation,
}

pub struct DiagnosticsLocation {
    pub line_number: usize,
    pub col_number: usize,
    pub line: Option<String>,
}

impl ParseError {
    pub fn new(
        start_offset: usize,
        end_offset: usize,
        source: &[u8],
        message: String,
    ) -> ParseError {
        let diagnostics = get_diagnostics_location(start_offset, source);
        ParseError {
            start_offset,
            end_offset,
            message,
            diagnostics,
        }
    }
}

fn get_diagnostics_location(offset: usize, source: &[u8]) -> DiagnosticsLocation {
    let before = &source[..offset];
    let after = &source[offset..];

    let line_start = before
        .iter()
        .rposition(|&b| b == b'\n')
        .map(|pos| pos + 1)
        .unwrap_or(0);

    let line_end = after
        .iter()
        .position(|&b| b == b'\n')
        .map(|pos| if pos == 0 { 0 } else { pos - 1 })
        .unwrap_or(source.len())
        + offset;

    let line_number = before.iter().filter(|&&b| b == b'\n').count() + 1;

    let col_number = offset - line_start;

    DiagnosticsLocation {
        line_number,
        col_number,
        line: String::from_utf8(source[line_start..line_end].to_vec()).ok(),
    }
}
