use std::fmt;

/// One problem in the help files, with where it is. A build lists them all.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HelpError {
    /// The file, relative to the help directory (`recipes.md`, `ORDER`).
    pub file: String,
    /// 1-based line, or 0 when the problem is the file as a whole.
    pub line: usize,
    pub message: String,
}

impl HelpError {
    pub fn new(file: &str, line: usize, message: impl Into<String>) -> Self {
        Self { file: file.to_string(), line, message: message.into() }
    }
}

impl fmt::Display for HelpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.line == 0 {
            write!(f, "{}: {}", self.file, self.message)
        } else {
            write!(f, "{}:{}: {}", self.file, self.line, self.message)
        }
    }
}

impl std::error::Error for HelpError {}
