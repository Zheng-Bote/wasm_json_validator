use std::fs;
use std::io::{self, Read};
use clap::Parser;

use wasm_json_validator::formats::{parse_input, InputFormat};
use wasm_json_validator::schema::SchemaValidator;

#[derive(Parser, Debug)]
#[command(
    name = "wasm_json_validator",
    version = "0.1.0",
    about = "Fast JSON-Schema payload validator for WASM/WASI and Spin (JSON, CSV, XLSX)"
)]
struct Cli {
    /// Path or inline string to the JSON Schema
    #[arg(short, long)]
    schema: String,

    /// Input payload file path (or '-' for stdin)
    #[arg(short, long, default_value = "-")]
    input: String,

    /// Input format (json, csv, xlsx). If omitted, inferred from filename.
    #[arg(short, long)]
    format: Option<String>,

    /// Output report format: 'json' or 'text'
    #[arg(long, default_value = "text")]
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    // 1. Load schema
    let schema_content = if let Ok(content) = fs::read_to_string(&args.schema) {
        content
    } else {
        args.schema.clone()
    };

    let validator = SchemaValidator::compile_str(&schema_content)?;

    // 2. Read input payload
    let (input_bytes, inferred_format) = if args.input == "-" {
        let mut buf = Vec::new();
        io::stdin().read_to_end(&mut buf)?;
        (buf, None)
    } else {
        let buf = fs::read(&args.input)?;
        let detected = InputFormat::detect_from_filename(&args.input);
        (buf, detected)
    };

    let format = match args.format.as_deref() {
        Some("json") => InputFormat::Json,
        Some("csv") => InputFormat::Csv,
        Some("xlsx") => InputFormat::Xlsx,
        Some(other) => {
            eprintln!("Error: Unknown format '{other}'. Supported: json, csv, xlsx");
            std::process::exit(2);
        }
        None => inferred_format.unwrap_or(InputFormat::Json),
    };

    // 3. Parse input into canonical JSON structure
    let instance = match parse_input(&input_bytes, format) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error parsing input: {e}");
            std::process::exit(2);
        }
    };

    // 4. Validate
    let report = validator.validate(&instance);

    if args.output == "json" {
        println!("{}", report.to_json_string()?);
    } else {
        println!("{}", report.to_terminal_summary());
    }

    if !report.valid {
        std::process::exit(1);
    }

    Ok(())
}
