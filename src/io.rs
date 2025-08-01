use std::io::{self, Write};

/// Trait for abstracting input operations to enable testing
pub trait InputReader {
    fn read_line(&mut self, buf: &mut String) -> io::Result<usize>;
}

/// Trait for abstracting output operations to enable testing
pub trait OutputWriter: Write {
    fn write_line(&mut self, s: &str) -> io::Result<()> {
        writeln!(self, "{}", s)
    }
}

/// Standard input reader implementation
pub struct StdinReader;

impl InputReader for StdinReader {
    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        io::stdin().read_line(buf)
    }
}

/// Standard output writer implementation
pub struct StdoutWriter;

impl OutputWriter for StdoutWriter {}

impl Write for StdoutWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        io::stdout().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        io::stdout().flush()
    }
}

/// Mock implementations for testing
pub struct MockInputReader {
    pub responses: Vec<String>,
    pub current_index: usize,
}

impl MockInputReader {
    pub fn new(responses: Vec<String>) -> Self {
        Self {
            responses,
            current_index: 0,
        }
    }
}

impl InputReader for MockInputReader {
    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        if self.current_index < self.responses.len() {
            let response = &self.responses[self.current_index];
            buf.push_str(response);
            if !response.ends_with('\n') {
                buf.push('\n');
            }
            self.current_index += 1;
            Ok(buf.len())
        } else {
            Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "No more mock responses available",
            ))
        }
    }
}

pub struct MockOutputWriter {
    pub output: Vec<String>,
}

impl Default for MockOutputWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl MockOutputWriter {
    pub fn new() -> Self {
        Self {
            output: Vec::new(),
        }
    }
}

impl OutputWriter for MockOutputWriter {}

impl Write for MockOutputWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let s = String::from_utf8_lossy(buf);
        self.output.push(s.to_string());
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
