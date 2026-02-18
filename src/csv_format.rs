//! CSV с заголовком.

use std::io::{BufRead, BufReader, Write, Read};
use crate::{Transaction, TxType, TxStatus, Result, ParserError};

/// Парсит одну строку CSV с учётом кавычек: внутри "..." запятые не разделяют поля.
fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for ch in line.chars() {
        match (in_quotes, ch) {
            (_, '"') => in_quotes = !in_quotes,
            (false, ',') => {
                fields.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    fields.push(current.trim().to_string());
    fields
}

/// Читаем CSV: заголовок, затем по строке на транзакцию.
pub fn from_read<R: Read>(reader: R) -> Result<Vec<Transaction>> {
    let mut transactions = Vec::new();
    let buf_reader = BufReader::new(reader);
    let mut lines = buf_reader.lines();

    if let Some(header_result) = lines.next() {
        header_result?;
    }

    for line_result in lines {
        let line = line_result?;
        if line.trim().is_empty() {
            continue;
        }

        let fields = parse_csv_line(&line);

        if fields.len() < 8 {
            return Err(ParserError::Format(format!("Недостаточно полей в CSV: {}", line)));
        }

        let description = fields[7]
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .unwrap_or(&fields[7])
            .to_string();

        let tx = Transaction {
            tx_id: fields[0].parse()?,
            tx_type: match fields[1].as_str() {
                "DEPOSIT" => TxType::Deposit,
                "TRANSFER" => TxType::Transfer,
                "WITHDRAWAL" => TxType::Withdrawal,
                _ => return Err(ParserError::Format(format!("Тип: {}", fields[1]))),
            },
            from_user_id: fields[2].parse()?,
            to_user_id: fields[3].parse()?,
            amount: fields[4].parse()?,
            timestamp: fields[5].parse()?,
            status: match fields[6].as_str() {
                "SUCCESS" => TxStatus::Success,
                "FAILURE" => TxStatus::Failure,
                "PENDING" => TxStatus::Pending,
                _ => return Err(ParserError::Format(format!("Статус: {}", fields[6]))),
            },
            description,
        };

        transactions.push(tx);
    }

    Ok(transactions)
}

/// Пишем CSV с заголовком.
pub fn write_to<W: Write>(mut writer: W, transactions: &[Transaction]) -> Result<()> {
    writeln!(writer, "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION")?;

    for tx in transactions {
        let type_str = match tx.tx_type {
            TxType::Deposit => "DEPOSIT",
            TxType::Transfer => "TRANSFER",
            TxType::Withdrawal => "WITHDRAWAL",
        };
        let status_str = match tx.status {
            TxStatus::Success => "SUCCESS",
            TxStatus::Failure => "FAILURE",
            TxStatus::Pending => "PENDING",
        };

        writeln!(
            writer,
            "{},{},{},{},{},{},{},\"{}\"",
            tx.tx_id, type_str, tx.from_user_id, tx.to_user_id, tx.amount, tx.timestamp, status_str, tx.description
        )?;
    }
    Ok(())
}
