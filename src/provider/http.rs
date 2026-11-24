use std::fs;
use serde_json::json;

use crate::formats::{parse_input, InputFormat};
use crate::schema::SchemaValidator;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl HttpResponse {
    pub fn ok_json(body: String) -> Self {
        Self {
            status: 200,
            headers: vec![("Content-Type".into(), "application/json".into())],
            body,
        }
    }

    pub fn bad_request(msg: &str) -> Self {
        let err_json = json!({
            "valid": false,
            "error": msg
        });
        Self {
            status: 400,
            headers: vec![("Content-Type".into(), "application/json".into())],
            body: err_json.to_string(),
        }
    }

    pub fn bad_request_json(body: String) -> Self {
        Self {
            status: 400,
            headers: vec![("Content-Type".into(), "application/json".into())],
            body,
        }
    }

    pub fn method_not_allowed() -> Self {
        let err_json = json!({
            "valid": false,
            "error": "Method Not Allowed. Only POST and PUT are supported."
        });
        Self {
            status: 405,
            headers: vec![("Content-Type".into(), "application/json".into())],
            body: err_json.to_string(),
        }
    }
}

pub fn handle_http_request(
    method: &str,
    _path: &str,
    query: &str,
    headers: &[(String, String)],
    body: &[u8],
) -> HttpResponse {
    // 1. Verify HTTP Method
    if method != "POST" && method != "PUT" {
        return HttpResponse::method_not_allowed();
    }

    // 2. Extract schema identifier from query param or header
    let schema_param = extract_query_param(query, "schema").or_else(|| {
        headers.iter().find_map(|(k, v)| {
            if k.eq_ignore_ascii_case("x-schema-id") || k.eq_ignore_ascii_case("x-schema") {
                Some(v.clone())
            } else {
                None
            }
        })
    });

    let schema_source = match schema_param {
        Some(s) if !s.trim().is_empty() => s,
        _ => {
            return HttpResponse::bad_request(
                "Missing schema parameter. Specify ?schema=<path_or_json> or X-Schema-ID header.",
            )
        }
    };

    // 3. Extract or infer format
    let format_param = extract_query_param(query, "format");
    let content_type = headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("content-type"))
        .map(|(_, v)| v.as_str());

    let format = if let Some(ref fmt) = format_param {
        match fmt.to_lowercase().as_str() {
            "csv" => InputFormat::Csv,
            "xlsx" => InputFormat::Xlsx,
            _ => InputFormat::Json,
        }
    } else if let Some(ct) = content_type {
        if ct.contains("text/csv") || ct.contains("csv") {
            InputFormat::Csv
        } else if ct.contains("spreadsheet") || ct.contains("xlsx") {
            InputFormat::Xlsx
        } else {
            InputFormat::Json
        }
    } else {
        InputFormat::Json
    };

    // 4. Load Schema
    let schema_str = if schema_source.trim_start().starts_with('{') {
        schema_source
    } else {
        match fs::read_to_string(&schema_source) {
            Ok(content) => content,
            Err(e) => {
                return HttpResponse::bad_request(&format!(
                    "Unable to read schema file '{schema_source}': {e}"
                ));
            }
        }
    };

    let validator = match SchemaValidator::compile_str(&schema_str) {
        Ok(v) => v,
        Err(e) => {
            return HttpResponse::bad_request(&format!("Schema compilation error: {e}"));
        }
    };

    // 5. Parse Payload
    let instance = match parse_input(body, format) {
        Ok(val) => val,
        Err(e) => {
            return HttpResponse::bad_request(&format!("Payload parsing error: {e}"));
        }
    };

    // 6. Validate
    let report = validator.validate(&instance);
    if report.valid {
        let resp = json!({
            "valid": true,
            "status": "Payload valid against schema."
        });
        HttpResponse::ok_json(resp.to_string())
    } else {
        match report.to_json_string() {
            Ok(json_body) => HttpResponse::bad_request_json(json_body),
            Err(e) => HttpResponse::bad_request(&format!("Internal reporting error: {e}")),
        }
    }
}

fn extract_query_param(query: &str, key: &str) -> Option<String> {
    for pair in query.split('&') {
        let mut parts = pair.splitn(2, '=');
        if let Some(k) = parts.next() {
            if k == key {
                return parts.next().map(|v| v.to_string());
            }
        }
    }
    None
}
