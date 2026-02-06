
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ParserError {
    Io(io::Error),
    Parse(String),
    Format(String),
    /// Невалидная UTF-8.
    Utf8(std::string::FromUtf8Error),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::Io(e) => write!(f, "Ошибка ввода-вывода: {}", e),
            ParserError::Parse(e) => write!(f, "Ошибка парсинга: {}", e),
            ParserError::Format(e) => write!(f, "Ошибка формата: {}", e),
            ParserError::Utf8(e) => write!(f, "Ошибка кодировки: {}", e),
        }
    }
}

impl std::error::Error for ParserError {}

impl From<io::Error> for ParserError {
    fn from(error: io::Error) -> Self {
        ParserError::Io(error)
    }
}

impl From<std::num::ParseIntError> for ParserError {
    fn from(error: std::num::ParseIntError) -> Self {
        ParserError::Parse(error.to_string())
    }
}

impl From<std::string::FromUtf8Error> for ParserError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        ParserError::Utf8(error)
    }
}

pub type Result<T> = std::result::Result<T, ParserError>;
