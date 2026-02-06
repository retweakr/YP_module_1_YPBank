use std::env;
use std::fs::File;
use parser::{text_format, csv_format, bin_format, Result, Transaction};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut file1_path = None;
    let mut format1 = None;
    let mut file2_path = None;
    let mut format2 = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--file1" => {
                if i + 1 < args.len() {
                    file1_path = Some(&args[i + 1]);
                    i += 1;
                }
            }
            "--format1" => {
                if i + 1 < args.len() {
                    format1 = Some(&args[i + 1]);
                    i += 1;
                }
            }
            "--file2" => {
                if i + 1 < args.len() {
                    file2_path = Some(&args[i + 1]);
                    i += 1;
                }
            }
            "--format2" => {
                if i + 1 < args.len() {
                    format2 = Some(&args[i + 1]);
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    let file1_path = file1_path.ok_or_else(|| parser::ParserError::Format("Укажите --file1".into()))?;
    let file2_path = file2_path.ok_or_else(|| parser::ParserError::Format("Укажите --file2".into()))?;
    
    let fmt1 = format1.map(|s| s.as_str()).unwrap_or("text");
    let fmt2 = format2.map(|s| s.as_str()).unwrap_or("text");

    let txs1 = load_transactions(file1_path, fmt1)?;
    let txs2 = load_transactions(file2_path, fmt2)?;

    if txs1.len() != txs2.len() {
        println!(
            "The number of transactions differs: {} in first file vs {} in second file.",
            txs1.len(),
            txs2.len()
        );
        return Ok(());
    }

    let mut identical = true;
    for (i, (t1, t2)) in txs1.iter().zip(txs2.iter()).enumerate() {
        if t1 != t2 {
            println!(
                "Transaction at index {} does not match (TX_ID: {}).",
                i, t1.tx_id
            );
            println!("  File 1: {:?}", t1);
            println!("  File 2: {:?}", t2);
            identical = false;
        }
    }

    if identical {
        println!(
            "The transaction records in '{}' and '{}' are identical.",
            file1_path, file2_path
        );
    }

    Ok(())
}

fn load_transactions(path: &str, format: &str) -> Result<Vec<Transaction>> {
    let file = File::open(path)?;
    match format {
        "csv" => csv_format::from_read(file),
        "bin" | "binary" => bin_format::from_read(file),
        "text" => text_format::from_read(file),
        _ => Err(parser::ParserError::Format(format!("Неизвестный формат: {}", format))),
    }
}
