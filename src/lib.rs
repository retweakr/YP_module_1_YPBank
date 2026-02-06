//! YPBank: парсинг CSV, текст, бинарник. Через [`Read`]/[`Write`].
//! [`Read`]: std::io::Read
//! [`Write`]: std::io::Write

pub mod error;
pub mod text_format;
pub mod csv_format;
pub mod bin_format;

pub use error::{ParserError, Result};

/// Тип операции.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxType {
    Deposit,
    Transfer,
    Withdrawal,
}

/// Статус транзакции.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxStatus {
    Success,
    Failure,
    Pending,
}

/// Транзакция.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub tx_id: u64,
    pub tx_type: TxType,
    /// 0 для пополнения извне.
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: i64,
    pub timestamp: u64,
    pub status: TxStatus,
    pub description: String,
}

impl Transaction {
    pub fn from_csv<R: std::io::Read>(reader: R) -> Result<Vec<Self>> {
        csv_format::from_read(reader)
    }

    pub fn to_csv<W: std::io::Write>(writer: W, transactions: &[Self]) -> Result<()> {
        csv_format::write_to(writer, transactions)
    }

    pub fn from_bin<R: std::io::Read>(reader: R) -> Result<Vec<Self>> {
        bin_format::from_read(reader)
    }
    pub fn to_bin<W: std::io::Write>(writer: W, transactions: &[Self]) -> Result<()> {
        bin_format::write_to(writer, transactions)
    }

    pub fn from_text<R: std::io::Read>(reader: R) -> Result<Vec<Self>> {
        text_format::from_read(reader)
    }

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
    fn test_csv_description_with_comma() -> Result<()> {
        let data = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n\
            1,DEPOSIT,0,2,100,1633036800000,SUCCESS,\"Pay, with comma\"";
        let cursor = Cursor::new(data);
        let txs = Transaction::from_csv(cursor)?;
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].description, "Pay, with comma");
        Ok(())
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
        match result {
            Err(ParserError::Format(msg)) => assert!(msg.contains("MAGIC")),
            Err(e) => panic!("Expected Format error, got {:?}", e),
            Ok(_) => panic!("Expected error for invalid MAGIC"),
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

    #[test]
    fn test_cross_format_roundtrip() -> Result<()> {
        let txs = create_test_txs();
        let mut buf = Vec::new();
        Transaction::to_csv(&mut buf, &txs)?;
        let from_csv = Transaction::from_csv(Cursor::new(&buf))?;
        buf.clear();
        Transaction::to_bin(&mut buf, &from_csv)?;
        let from_bin = Transaction::from_bin(Cursor::new(&buf))?;
        buf.clear();
        Transaction::to_text(&mut buf, &from_bin)?;
        let from_text = Transaction::from_text(Cursor::new(&buf))?;
        assert_eq!(txs, from_text);
        Ok(())
    }
}