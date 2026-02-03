use std::fmt;
use std::io;

/// Наш собственный тип ошибки. 
/// В Rust ошибки — это не исключения, а просто значения.
#[derive(Debug)]
pub enum ParserError {
    Io(io::Error),      // Ошибки ввода-вывода (например, файл не найден)
    Parse(String),     // Ошибки парсинга чисел
    Format(String),    // Ошибки в структуре самого текстового файла
    Utf8(std::string::FromUtf8Error),
}

// Позволяет выводить ошибку в консоль
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

// Делает наш тип совместимым со стандартными ошибками Rust
impl std::error::Error for ParserError {}

// Эти "impl From" позволяют автоматически превращать стандартные ошибки в наши
// когда мы используем оператор `?`
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

// Удобный алиас для Result, чтобы не писать везде ошибку вручную
pub type Result<T> = std::result::Result<T, ParserError>;
