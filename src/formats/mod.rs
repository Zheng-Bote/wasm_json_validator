//! Multi-format ingestion layer (JSON, CSV, XLSX).

use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FormatError {
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("CSV parse error: {0}")]
    Csv(#[from] csv::Error),
    #[error("XLSX parse error: {0}")]
    Xlsx(String),
    #[error("Unsupported or unrecognized format: {0}")]
    UnsupportedFormat(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFormat {
    Json,
    Csv,
    Xlsx,
}

impl InputFormat {
    pub fn detect_from_filename(filename: &str) -> Option<Self> {
        let lower = filename.to_lowercase();
        if lower.ends_with(".json") {
            Some(InputFormat::Json)
        } else if lower.ends_with(".csv") {
            Some(InputFormat::Csv)
        } else if lower.ends_with(".xlsx") {
            Some(InputFormat::Xlsx)
        } else {
            None
        }
    }
}

pub fn parse_input(bytes: &[u8], format: InputFormat) -> Result<Value, FormatError> {
    match format {
        InputFormat::Json => {
            let val: Value = serde_json::from_slice(bytes)?;
            Ok(val)
        }
        InputFormat::Csv => {
            let mut reader = csv::Reader::from_reader(bytes);
            let headers = reader.headers()?.clone();
            let mut rows = Vec::new();

            for result in reader.records() {
                let record = result?;
                let mut map = serde_json::Map::new();
                for (header, field) in headers.iter().zip(record.iter()) {
                    let field_val = if let Ok(num) = field.parse::<i64>() {
                        Value::Number(num.into())
                    } else if let Ok(flt) = field.parse::<f64>() {
                        serde_json::Number::from_f64(flt)
                            .map(Value::Number)
                            .unwrap_or_else(|| Value::String(field.to_string()))
                    } else if field.eq_ignore_ascii_case("true") {
                        Value::Bool(true)
                    } else if field.eq_ignore_ascii_case("false") {
                        Value::Bool(false)
                    } else {
                        Value::String(field.to_string())
                    };
                    map.insert(header.to_string(), field_val);
                }
                rows.push(Value::Object(map));
            }
            Ok(Value::Array(rows))
        }
        InputFormat::Xlsx => {
            use calamine::{Reader, Xlsx};
            use std::io::Cursor;

            let cursor = Cursor::new(bytes);
            let mut workbook: Xlsx<_> = Xlsx::new(cursor)
                .map_err(|e| FormatError::Xlsx(e.to_string()))?;

            let sheet_names = workbook.sheet_names();
            if sheet_names.is_empty() {
                return Err(FormatError::Xlsx("No sheets found in XLSX workbook".to_string()));
            }

            let first_sheet_name = sheet_names[0].clone();
            let range = workbook
                .worksheet_range(&first_sheet_name)
                .map_err(|e| FormatError::Xlsx(e.to_string()))?;

            let mut rows_iter = range.rows();
            let header_row = match rows_iter.next() {
                Some(r) => r,
                None => return Ok(Value::Array(vec![])),
            };

            let headers: Vec<String> = header_row.iter().map(|c| c.to_string()).collect();
            let mut items = Vec::new();

            for row in rows_iter {
                let mut map = serde_json::Map::new();
                for (idx, cell) in row.iter().enumerate() {
                    let header = headers.get(idx).cloned().unwrap_or_else(|| format!("col_{idx}"));
                    let val = match cell {
                        calamine::Data::Int(v) => Value::Number((*v).into()),
                        calamine::Data::Float(v) => serde_json::Number::from_f64(*v)
                            .map(Value::Number)
                            .unwrap_or(Value::Null),
                        calamine::Data::String(s) => Value::String(s.clone()),
                        calamine::Data::Bool(b) => Value::Bool(*b),
                        calamine::Data::DateTime(dt) => Value::String(dt.to_string()),
                        calamine::Data::DateTimeIso(s) => Value::String(s.clone()),
                        calamine::Data::DurationIso(s) => Value::String(s.clone()),
                        calamine::Data::Empty => Value::Null,
                        calamine::Data::Error(e) => Value::String(format!("ERR: {e:?}")),
                    };
                    map.insert(header, val);
                }
                items.push(Value::Object(map));
            }

            Ok(Value::Array(items))
        }
    }
}

#[cfg(test)]
mod formats_test;

