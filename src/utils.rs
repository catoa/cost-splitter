use crate::errors::Result;
use crate::io::InputReader;
use std::io;

/// Read input from any reader that implements InputReader trait
pub fn read_input<R: InputReader>(reader: &mut R, buf: &mut String) -> Result<usize> {
    reader.read_line(buf).map_err(Into::into)
}

/// Legacy function for backwards compatibility - prefer using read_input with dependency injection
pub fn read_from_stdin(msg: &mut String, error_msg: &str) {
    io::stdin().read_line(msg).expect(error_msg);
}

/// Split a string by delimiter and filter out empty values
pub fn get_values_by_delim(s: String, delim: &str) -> Vec<String> {
    s.split(delim)
        .filter_map(|val| {
            let trimmed = val.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_owned())
            }
        })
        .collect()
}
