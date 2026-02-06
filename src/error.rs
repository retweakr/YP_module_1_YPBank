//! Типы ошибок парсера финансовых данных YPBank.

use std::fmt;
use std::io;

/// Ошибка парсинга или сериализации данных.
///
/// Используется во всех операциях чтения/записи форматов YPBankCsv, YPBankText и YPBankBin.
#[derive(Debug)]
pub enum ParserError {
    /// Ошибка ввода-вывода (файл не найден, нет прав, обрыв потока и т.д.).
    Io(io::Error),
    /// Ошибка разбора числа или неверный тип/статус.
    Parse(String),
    /// Нарушение структуры формата (неверный заголовок, недостаточно полей, неизвестное значение).
    Format(String),
    /// Некорректная UTF-8 последовательность (например, в описании транзакции).
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

/// Алиас для `Result<T, ParserError>` — результат операций парсера.
pub type Result<T> = std::result::Result<T, ParserError>;
