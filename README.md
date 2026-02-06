# YPBank Financial Tools

Набор инструментов для работы с финансовыми данными YPBank. Проект включает в себя библиотеку для парсинга и два консольных приложения.

## Как запустить проект

### Требования

- [Rust](https://www.rust-lang.org/) (рекомендуется установка через rustup).

### Установка Rust (если ещё не установлен)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
```

### Сборка

В корне проекта выполните:

```bash
cargo build --release
```

Будут собраны библиотека **parser**, утилита **ypbank_converter** и утилита **ypbank_compare**. Исполняемые файлы появятся в `target/release/`.

### Запуск утилит

**Конвертер** — переводит файл из одного формата в другой (результат в stdout):

```bash
cargo run --bin ypbank_converter -- --input examples/records_example.csv --input-format csv --output-format text
```

Сохранение результата в файл:

```bash
cargo run --bin ypbank_converter -- --input examples/records_example.csv --input-format csv --output-format bin > output.bin
```

**Сравнение двух файлов** (форматы файлов могут быть разными):

```bash
cargo run --bin ypbank_compare -- --file1 examples/records_example.bin --format1 bin --file2 examples/records_example.csv --format2 csv
```

После `cargo build --release` можно вызывать утилиты напрямую:

```bash
./target/release/ypbank_converter --input examples/records_example.csv --input-format csv --output-format text
./target/release/ypbank_compare --file1 examples/records_example.bin --format1 bin --file2 examples/records_example.csv --format2 csv
```

### Форматы

Для `--input-format` / `--output-format` и `--format1` / `--format2` допустимы значения: `csv`, `text`, `bin` (или `binary`). Примеры данных лежат в папке `examples/`.

### Тестирование

```bash
cargo test
```

---

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
