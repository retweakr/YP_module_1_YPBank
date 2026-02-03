pub mod error;
pub mod text_format;
pub mod csv_format;
pub mod bin_format;

// Реэкспорт для удобства
pub use error::{ParserError, Result};

/// Тип финансовой операции.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxType {
    /// Пополнение счета.
    Deposit,
    /// Перевод между счетами.
    Transfer,
    /// Снятие средств.
    Withdrawal,
}

/// Статус выполнения транзакции.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxStatus {
    /// Успешно завершена.
    Success,
    /// Завершилась ошибкой.
    Failure,
    /// В ожидании обработки.
    Pending,
}

/// Основная структура транзакции, содержащая всю информацию о финансовой операции.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    /// Уникальный идентификатор транзакции.
    pub tx_id: u64,
    /// Тип транзакции.
    pub tx_type: TxType,
    /// ID отправителя (0 для внешних пополнений).
    pub from_user_id: u64,
    /// ID получателя.
    pub to_user_id: u64,
    /// Сумма транзакции в минимальных единицах валюты (например, копейки).
    pub amount: i64,
    /// Временная метка транзакции (Unix timestamp в миллисекундах).
    pub timestamp: u64,
    /// Текущий статус транзакции.
    pub status: TxStatus,
    /// Произвольное текстовое описание транзакции.
    pub description: String,
}

impl Transaction {
    /// Читает список транзакций из CSV формата.
    pub fn from_csv<R: std::io::Read>(reader: R) -> Result<Vec<Self>> {
        csv_format::from_read(reader)
    }

    /// Записывает список транзакций в CSV формате.
    pub fn to_csv<W: std::io::Write>(writer: W, transactions: &[Self]) -> Result<()> {
        csv_format::write_to(writer, transactions)
    }

    /// Читает список транзакций из бинарного формата.
    pub fn from_bin<R: std::io::Read>(reader: R) -> Result<Vec<Self>> {
        bin_format::from_read(reader)
    }

    /// Записывает список транзакций в бинарном формате.
    pub fn to_bin<W: std::io::Write>(writer: W, transactions: &[Self]) -> Result<()> {
        bin_format::write_to(writer, transactions)
    }

    /// Читает список транзакций из текстового формата.
    pub fn from_text<R: std::io::Read>(reader: R) -> Result<Vec<Self>> {
        text_format::from_read(reader)
    }

    /// Записывает список транзакций в текстовом формате.
    pub fn to_text<W: std::io::Write>(writer: W, transactions: &[Self]) -> Result<()> {
        text_format::write_to(writer, transactions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn create_test_txs() -> Vec<Transaction> {
        vec![
            Transaction {
                tx_id: 1001,
                tx_type: TxType::Deposit,
                from_user_id: 0,
                to_user_id: 501,
                amount: 50000,
                timestamp: 1672531200000,
                status: TxStatus::Success,
                description: "Initial funding".to_string(),
            },
        ]
    }

    #[test]
    fn test_text_roundtrip() -> Result<()> {
        let txs = create_test_txs();
        let mut buffer = Vec::new();
        Transaction::to_text(&mut buffer, &txs)?;
        let cursor = Cursor::new(buffer);
        let decoded = Transaction::from_text(cursor)?;
        assert_eq!(txs, decoded);
        Ok(())
    }

    #[test]
    fn test_csv_roundtrip() -> Result<()> {
        let txs = create_test_txs();
        let mut buffer = Vec::new();
        Transaction::to_csv(&mut buffer, &txs)?;
        let cursor = Cursor::new(buffer);
        let decoded = Transaction::from_csv(cursor)?;
        assert_eq!(txs, decoded);
        Ok(())
    }

    #[test]
    fn test_bin_roundtrip() -> Result<()> {
        let txs = create_test_txs();
        let mut buffer = Vec::new();
        Transaction::to_bin(&mut buffer, &txs)?;
        let cursor = Cursor::new(buffer);
        let decoded = Transaction::from_bin(cursor)?;
        assert_eq!(txs, decoded);
        Ok(())
    }

    #[test]
    fn test_invalid_magic_bin() {
        let data = b"NOT_MAGIC_12345678";
        let cursor = Cursor::new(data);
        let result = Transaction::from_bin(cursor);
        assert!(result.is_err());
        if let Err(ParserError::Format(msg)) = result {
            assert!(msg.contains("MAGIC"));
        } else {
            panic!("Expected format error");
        }
    }

    #[test]
    fn test_csv_missing_fields() {
        let data = "TX_ID,TX_TYPE\n1001,DEPOSIT";
        let cursor = Cursor::new(data);
        let result = Transaction::from_csv(cursor);
        assert!(result.is_err());
    }

    #[test]
    fn test_text_missing_required_field() {
        let data = "TX_ID: 1001\nTX_TYPE: DEPOSIT\n\n";
        let cursor = Cursor::new(data);
        let result = Transaction::from_text(cursor);
        assert!(result.is_err());
    }
}