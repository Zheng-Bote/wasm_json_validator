use super::*;
use serde_json::json;

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
