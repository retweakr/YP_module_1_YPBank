pub mod error;
pub mod text_format;
pub mod csv_format;
pub mod bin_format;

// Реэкспорт для удобства
pub use error::{ParserError, Result};

/// Тип транзакции
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxType {
    Deposit,
    Transfer,
    Withdrawal,
}

/// Статус транзакции
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxStatus {
    Success,
    Failure,
    Pending,
}

/// Основная структура транзакции.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub tx_id: u64,
    pub tx_type: TxType,
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: i64,
    pub timestamp: u64,
    pub status: TxStatus,
    pub description: String,
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
        text_format::write_to(&mut buffer, &txs)?;
        let cursor = Cursor::new(buffer);
        let decoded = text_format::from_read(cursor)?;
        assert_eq!(txs, decoded);
        Ok(())
    }

    #[test]
    fn test_csv_roundtrip() -> Result<()> {
        let txs = create_test_txs();
        let mut buffer = Vec::new();
        csv_format::write_to(&mut buffer, &txs)?;
        let cursor = Cursor::new(buffer);
        let decoded = csv_format::from_read(cursor)?;
        assert_eq!(txs, decoded);
        Ok(())
    }

    #[test]
    fn test_bin_roundtrip() -> Result<()> {
        let txs = create_test_txs();
        let mut buffer = Vec::new();
        bin_format::write_to(&mut buffer, &txs)?;
        let cursor = Cursor::new(buffer);
        let decoded = bin_format::from_read(cursor)?;
        assert_eq!(txs, decoded);
        Ok(())
    }
}