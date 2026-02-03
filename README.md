# YPBank Financial Tools

Набор инструментов для работы с финансовыми данными YPBank. Проект включает в себя библиотеку для парсинга и два консольных приложения.

## Структура проекта

- `src/lib.rs` — Библиотека (crate `parser`) для чтения и записи финансовых данных в форматах CSV, Text и Binary.
- `src/main.rs` — Консольная утилита `converter` для конвертации данных.
- `src/bin/comparer.rs` — Консольная утилита `comparer` для сравнения двух файлов с транзакциями.
- `src/*.rs` — Модули реализации конкретных форматов и ошибок.

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

### Converter

Конвертация файла из одного формата в другой:

```bash
cargo run --bin converter -- --input examples/records_example.csv --input-format csv --output-format bin > output.bin
```

Параметры:
- `--input <path>`: Путь к входному файлу.
- `--input-format <format>`: Формат входного файла (`csv`, `text`, `bin`).
- `--output-format <format>`: Формат выходного файла (`csv`, `text`, `bin`).

### Comparer

Сравнение двух файлов:

```bash
cargo run --bin comparer -- --file1 file1.csv --format1 csv --file2 file2.bin --format2 bin
```

Параметры:
- `--file1 <path>`, `--file2 <path>`: Пути к файлам.
- `--format1 <format>`, `--format2 <format>`: Форматы файлов.

## Тестирование

Запуск всех тестов проекта:

```bash
cargo test
```
