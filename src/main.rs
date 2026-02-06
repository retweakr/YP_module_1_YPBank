use std::env;
use std::fs::File;
use std::io;
use parser::{text_format, csv_format, bin_format, Result};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut input_file = None;
    let mut input_format = None;
    let mut output_format = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--input" => {
                if i + 1 < args.len() {
                    input_file = Some(&args[i + 1]);
                    i += 1;
                }
            }
            "--input-format" => {
                if i + 1 < args.len() {
                    input_format = Some(&args[i + 1]);
                    i += 1;
                }
            }
            "--output-format" => {
                if i + 1 < args.len() {
                    output_format = Some(&args[i + 1]);
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    let input_path = input_file.ok_or_else(|| {
        parser::ParserError::Format("Ошибка: Используйте --input <файл>".into())
    })?;
    
    let in_fmt = input_format.map(|s| s.as_str()).unwrap_or("text");
    let out_fmt = output_format.map(|s| s.as_str()).unwrap_or("text");

    let file = File::open(input_path)?;
    let transactions = match in_fmt {
        "csv" => csv_format::from_read(file)?,
        "bin" | "binary" => bin_format::from_read(file)?,
        "text" => text_format::from_read(file)?,
        _ => return Err(parser::ParserError::Format(format!("Неизвестный входной формат: {}", in_fmt))),
    };

    let stdout = io::stdout();
    let handle = stdout.lock();

    match out_fmt {
        "csv" => csv_format::write_to(handle, &transactions)?,
        "bin" | "binary" => bin_format::write_to(handle, &transactions)?,
        "text" => text_format::write_to(handle, &transactions)?,
        _ => return Err(parser::ParserError::Format(format!("Неизвестный выходной формат: {}", out_fmt))),
    }

    Ok(())
}
