//! Модуль для работы с текстовым форматом YPBankText.
//!
//! Текстовый формат представляет транзакции в виде блоков "ключ: значение".

use std::io::{BufRead, BufReader, Write, Read};
use crate::{Transaction, TxType, TxStatus, Result, ParserError};

/// Читает транзакции из текстового потока.
pub fn from_read<R: Read>(reader: R) -> Result<Vec<Transaction>> {
    let mut transactions = Vec::new();
    
    // BufReader делает чтение более эффективным (читает не по байту, а кусками)
    let buf_reader = BufReader::new(reader);
    let mut current_block = Vec::new();

    for line_result in buf_reader.lines() {
        // В Rust чтение может вернуть ошибку (например, диск выдернули), 
        // поэтому используем `?` — если ошибка, выходим из функции и возвращаем её.
        let line = line_result?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            // Если пустая строка — значит блок закончился, парсим его
            if !current_block.is_empty() {
                transactions.push(parse_block(&current_block)?);
                current_block.clear();
            }
        } else if !trimmed.starts_with('#') {
            // Игнорируем комментарии и собираем строки блока
            current_block.push(line);
        }
    }

    // Не забываем последний блок, если файл не закончился пустой строкой
    if !current_block.is_empty() {
        transactions.push(parse_block(&current_block)?);
    }

    Ok(transactions)
}

/// Внутренняя функция для превращения списка строк "ключ: значение" в структуру Transaction.
fn parse_block(lines: &[String]) -> Result<Transaction> {
    // В JS мы бы создали пустой объект, в Rust мы используем Option, 
    // чтобы гарантировать, что все поля будут заполнены.
    let mut tx_id = None;
    let mut tx_type = None;
    let mut from_user_id = None;
    let mut to_user_id = None;
    let mut amount = None;
    let mut timestamp = None;
    let mut status = None;
    let mut description = None;

    for line in lines {
        // Разделяем строку по первому символу ':'
        let (key, value) = line.split_once(':')
            .ok_or_else(|| ParserError::Format(format!("Неверная строка: {}", line)))?;
        
        let key = key.trim();
        let value = value.trim();

        match key {
            "TX_ID" => tx_id = Some(value.parse::<u64>()?),
            "TX_TYPE" => tx_type = Some(match value {
                "DEPOSIT" => TxType::Deposit,
                "TRANSFER" => TxType::Transfer,
                "WITHDRAWAL" => TxType::Withdrawal,
                _ => return Err(ParserError::Format(format!("Неизвестный тип: {}", value))),
            }),
            "FROM_USER_ID" => from_user_id = Some(value.parse::<u64>()?),
            "TO_USER_ID" => to_user_id = Some(value.parse::<u64>()?),
            "AMOUNT" => amount = Some(value.parse::<i64>()?),
            "TIMESTAMP" => timestamp = Some(value.parse::<u64>()?),
            "STATUS" => status = Some(match value {
                "SUCCESS" => TxStatus::Success,
                "FAILURE" => TxStatus::Failure,
                "PENDING" => TxStatus::Pending,
                _ => return Err(ParserError::Format(format!("Неизвестный статус: {}", value))),
            }),
            "DESCRIPTION" => description = Some(value.trim_matches('"').to_string()),
            _ => {} // Неизвестные поля просто игнорируем
        }
    }

    // Собираем структуру. Если какого-то поля не хватает, возвращаем ошибку.
    // Это гораздо надежнее, чем undefined в JS.
    Ok(Transaction {
        tx_id: tx_id.ok_or_else(|| ParserError::Format("Отсутствует TX_ID".into()))?,
        tx_type: tx_type.ok_or_else(|| ParserError::Format("Отсутствует TX_TYPE".into()))?,
        from_user_id: from_user_id.ok_or_else(|| ParserError::Format("Отсутствует FROM_USER_ID".into()))?,
        to_user_id: to_user_id.ok_or_else(|| ParserError::Format("Отсутствует TO_USER_ID".into()))?,
        amount: amount.ok_or_else(|| ParserError::Format("Отсутствует AMOUNT".into()))?,
        timestamp: timestamp.ok_or_else(|| ParserError::Format("Отсутствует TIMESTAMP".into()))?,
        status: status.ok_or_else(|| ParserError::Format("Отсутствует STATUS".into()))?,
        description: description.ok_or_else(|| ParserError::Format("Отсутствует DESCRIPTION".into()))?,
    })
}

/// Записывает транзакции в текстовый поток.
pub fn write_to<W: Write>(mut writer: W, transactions: &[Transaction]) -> Result<()> {
    for (i, tx) in transactions.iter().enumerate() {
        writeln!(writer, "# Запись {}", i + 1)?;
        writeln!(writer, "TX_ID: {}", tx.tx_id)?;
        writeln!(writer, "TX_TYPE: {}", match tx.tx_type {
            TxType::Deposit => "DEPOSIT",
            TxType::Transfer => "TRANSFER",
            TxType::Withdrawal => "WITHDRAWAL",
        })?;
        writeln!(writer, "FROM_USER_ID: {}", tx.from_user_id)?;
        writeln!(writer, "TO_USER_ID: {}", tx.to_user_id)?;
        writeln!(writer, "AMOUNT: {}", tx.amount)?;
        writeln!(writer, "TIMESTAMP: {}", tx.timestamp)?;
        writeln!(writer, "STATUS: {}", match tx.status {
            TxStatus::Success => "SUCCESS",
            TxStatus::Failure => "FAILURE",
            TxStatus::Pending => "PENDING",
        })?;
        writeln!(writer, "DESCRIPTION: \"{}\"", tx.description)?;
        writeln!(writer)?; // Пустая строка между блоками
    }
    Ok(())
}
