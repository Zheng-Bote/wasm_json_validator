//! Structured validation report representation and formatting.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationErrorItem {
    pub instance_path: String,
    pub schema_path: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub valid: bool,
    pub error_count: usize,
    pub errors: Vec<ValidationErrorItem>,
}

impl ValidationReport {
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn to_terminal_summary(&self) -> String {
        if self.valid {
            "✓ Validation PASSED: Payload is valid against schema.".to_string()
        } else {
            let mut out = format!("✗ Validation FAILED ({} error(s)):\n", self.error_count);
            for (idx, err) in self.errors.iter().enumerate() {
                out.push_str(&format!(
                    "  {}. Path: `{}` | Schema: `{}`\n     Error: {}\n",
                    idx + 1,
                    err.instance_path,
                    err.schema_path,
                    err.message
                ));
            }
            out
        }
    }
}
