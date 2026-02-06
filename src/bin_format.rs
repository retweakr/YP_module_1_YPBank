//! Бинарник: YPBN, размер, поля (big-endian).

use std::io::{Write, Read};
use crate::{Transaction, TxType, TxStatus, Result, ParserError};

/// Сигнатура записи.
const MAGIC: &[u8; 4] = b"YPBN";

/// Читаем записи до EOF.
pub fn from_read<R: Read>(mut reader: R) -> Result<Vec<Transaction>> {
    let mut transactions = Vec::new();

    loop {
        let mut magic_buf = [0u8; 4];
        if let Err(e) = reader.read_exact(&mut magic_buf) {
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                break;
            }
            return Err(e.into());
        }

        if &magic_buf != MAGIC {
            return Err(ParserError::Format("Неверный заголовок MAGIC".into()));
        }

        let mut size_buf = [0u8; 4];
        reader.read_exact(&mut size_buf)?;

        let mut tx_id_buf = [0u8; 8];
        reader.read_exact(&mut tx_id_buf)?;
        let tx_id = u64::from_be_bytes(tx_id_buf);

        let mut type_buf = [0u8; 1];
        reader.read_exact(&mut type_buf)?;
        let tx_type = match type_buf[0] {
            0 => TxType::Deposit,
            1 => TxType::Transfer,
            2 => TxType::Withdrawal,
            _ => return Err(ParserError::Format("Неверный тип в бинарном файле".into())),
        };

        let mut from_buf = [0u8; 8];
        reader.read_exact(&mut from_buf)?;
        let from_user_id = u64::from_be_bytes(from_buf);

        let mut to_buf = [0u8; 8];
        reader.read_exact(&mut to_buf)?;
        let to_user_id = u64::from_be_bytes(to_buf);

        let mut amount_buf = [0u8; 8];
        reader.read_exact(&mut amount_buf)?;
        let amount = i64::from_be_bytes(amount_buf);

        let mut ts_buf = [0u8; 8];
        reader.read_exact(&mut ts_buf)?;
        let timestamp = u64::from_be_bytes(ts_buf);

        let mut status_buf = [0u8; 1];
        reader.read_exact(&mut status_buf)?;
        let status = match status_buf[0] {
            0 => TxStatus::Success,
            1 => TxStatus::Failure,
            2 => TxStatus::Pending,
            _ => return Err(ParserError::Format("Неверный статус в бинарном файле".into())),
        };

        let mut desc_len_buf = [0u8; 4];
        reader.read_exact(&mut desc_len_buf)?;
        let desc_len = u32::from_be_bytes(desc_len_buf) as usize;

        let mut desc_bytes = vec![0u8; desc_len];
        reader.read_exact(&mut desc_bytes)?;
        let description = String::from_utf8(desc_bytes)?;

        transactions.push(Transaction {
            tx_id,
            tx_type,
            from_user_id,
            to_user_id,
            amount,
            timestamp,
            status,
            description,
        });
    }

    Ok(transactions)
}

/// MAGIC + размер + поля (big-endian).
pub fn write_to<W: Write>(mut writer: W, transactions: &[Transaction]) -> Result<()> {
    for tx in transactions {
        writer.write_all(MAGIC)?;

        let desc_bytes = tx.description.as_bytes();
        let body_size = (8 + 1 + 8 + 8 + 8 + 8 + 1 + 4 + desc_bytes.len()) as u32;
        writer.write_all(&body_size.to_be_bytes())?;

        writer.write_all(&tx.tx_id.to_be_bytes())?;
        writer.write_all(&[match tx.tx_type {
            TxType::Deposit => 0,
            TxType::Transfer => 1,
            TxType::Withdrawal => 2,
        }])?;
        writer.write_all(&tx.from_user_id.to_be_bytes())?;
        writer.write_all(&tx.to_user_id.to_be_bytes())?;
        writer.write_all(&tx.amount.to_be_bytes())?;
        writer.write_all(&tx.timestamp.to_be_bytes())?;
        writer.write_all(&[match tx.status {
            TxStatus::Success => 0,
            TxStatus::Failure => 1,
            TxStatus::Pending => 2,
        }])?;
        writer.write_all(&(desc_bytes.len() as u32).to_be_bytes())?;
        writer.write_all(desc_bytes)?;
    }
    Ok(())
}
