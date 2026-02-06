# YPBank Financial Tools

Набор инструментов для работы с финансовыми данными YPBank. Проект включает в себя библиотеку для парсинга и два консольных приложения.

## Структура проекта

- `src/lib.rs` — Библиотека (crate `parser`) для чтения и записи финансовых данных в форматах CSV, Text и Binary.
- `src/main.rs` — Консольная утилита **ypbank_converter** для конвертации данных.
- `src/bin/comparer.rs` — Консольная утилита **ypbank_compare** для сравнения двух файлов с транзакциями.
- `src/error.rs` — Типы ошибок парсера.
- `src/csv_format.rs`, `src/text_format.rs`, `src/bin_format.rs` — Реализация форматов YPBankCsv, YPBankText, YPBankBin.

## Поддерживаемые форматы

- **YPBankCsv** — Таблица банковских операций в формате CSV.
- **YPBankText** — Текстовый формат описания списка операций.
- **YPBankBin** — Бинарное представление списка операций.

## Сборка

Для сборки всех компонентов проекта выполните:

```bash
cargo build --release
```

## Использование

### ypbank_converter

Конвертация файла из одного формата в другой. Результат выводится в stdout (перенаправьте в файл при необходимости).

```bash
ypbank_converter --input <input_file> --input-format <format> --output-format <format> > output_file.txt
```

Пример:

```bash
cargo run --bin ypbank_converter -- --input examples/records_example.csv --input-format csv --output-format bin > output.bin
```

Параметры:
- `--input <path>`: Путь к входному файлу.
- `--input-format <format>`: Формат входного файла (`csv`, `text`, `bin`).
- `--output-format <format>`: Формат выходного файла (`csv`, `text`, `bin`).

### ypbank_compare

Сравнение двух файлов с транзакциями. Входные файлы могут быть в любых поддерживаемых форматах.

```bash
ypbank_compare --file1 records_example.bin --format1 binary --file2 records_example.csv --format2 csv
```

Пример вывода при совпадении:  
`The transaction records in 'records_example.bin' and 'records_example.csv' are identical.`

При несовпадении выводится индекс и TX_ID расходящейся транзакции.

Параметры:
- `--file1 <path>`, `--file2 <path>`: Пути к файлам.
- `--format1 <format>`, `--format2 <format>`: Форматы файлов (`csv`, `text`, `bin`).

## Тестирование

Запуск всех тестов проекта:

```bash
cargo test
```
