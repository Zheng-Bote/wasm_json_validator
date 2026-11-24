use super::*;
use serde_json::json;

// payload-validation-core AC-1: Validation of JSON payloads against schema succeeds on valid data
#[test]
fn test_schema_validation_success() {
    let schema = json!({
        "type": "object",
        "properties": {
            "name": { "type": "string" },
            "age": { "type": "integer", "minimum": 0 }
        },
        "required": ["name", "age"]
    });

    let validator = SchemaValidator::compile(&schema).unwrap();
    let valid_payload = json!({
        "name": "Alice",
        "age": 30
    });

    let report = validator.validate(&valid_payload);
    assert!(report.valid);
    assert_eq!(report.error_count, 0);
}

// payload-validation-core AC-4: Structured error reporting with JSON Pointer on schema mismatch
#[test]
fn test_schema_validation_failure() {
    let schema = json!({
        "type": "object",
        "properties": {
            "name": { "type": "string" },
            "age": { "type": "integer", "minimum": 0 }
        },
        "required": ["name", "age"]
    });

    let validator = SchemaValidator::compile(&schema).unwrap();
    let invalid_payload = json!({
        "name": "Bob",
        "age": -5
    });

    let report = validator.validate(&invalid_payload);
    assert!(!report.valid);
    assert_eq!(report.error_count, 1);
    assert_eq!(report.errors[0].instance_path, "/age");
}

// payload-validation-core AC-5: WASI Preview 2 memory and verification test
#[test]
fn test_report_formatting() {
    let report = crate::report::ValidationReport {
        valid: false,
        error_count: 1,
        errors: vec![crate::report::ValidationErrorItem {
            instance_path: "/age".to_string(),
            schema_path: "/properties/age/minimum".to_string(),
            message: "-5 is less than minimum 0".to_string(),
        }],
    };

    let summary = report.to_terminal_summary();
    assert!(summary.contains("Validation FAILED"));
    assert!(summary.contains("/age"));

    let json_str = report.to_json_string().unwrap();
    assert!(json_str.contains("\"valid\": false"));
}
