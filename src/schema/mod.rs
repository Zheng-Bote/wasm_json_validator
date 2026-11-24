//! Schema compilation, registry, and validation execution.

use jsonschema::Validator;
use serde_json::Value;
use thiserror::Error;

use crate::report::ValidationReport;

#[derive(Debug, Error)]
pub enum SchemaError {
    #[error("Schema compilation error: {0}")]
    Compilation(String),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub struct SchemaValidator {
    validator: Validator,
}

impl SchemaValidator {
    pub fn compile(schema: &Value) -> Result<Self, SchemaError> {
        let validator = Validator::new(schema)
            .map_err(|e| SchemaError::Compilation(e.to_string()))?;
        Ok(Self { validator })
    }

    pub fn compile_str(schema_str: &str) -> Result<Self, SchemaError> {
        let val: Value = serde_json::from_str(schema_str)?;
        Self::compile(&val)
    }

    pub fn validate(&self, instance: &Value) -> ValidationReport {
        let mut errors = Vec::new();
        for error in self.validator.iter_errors(instance) {
            errors.push(crate::report::ValidationErrorItem {
                instance_path: error.instance_path.to_string(),
                schema_path: error.schema_path.to_string(),
                message: error.to_string(),
            });
        }

        let is_valid = errors.is_empty();
        ValidationReport {
            valid: is_valid,
            error_count: errors.len(),
            errors,
        }
    }
}

#[cfg(test)]
mod schema_test;

