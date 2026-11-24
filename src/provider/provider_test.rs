use super::http::{handle_http_request, HttpResponse};

// spin-http-gateway AC-1: Spin HTTP component accepts POST requests with ?schema query parameter
// spin-http-gateway AC-2: Valid requests return HTTP 200 OK with confirmation JSON
#[test]
fn test_http_post_valid_payload() {
    let schema = r#"{
        "type": "object",
        "properties": { "name": { "type": "string" } },
        "required": ["name"]
    }"#;
    let body = br#"{"name": "Alice"}"#;
    let query = format!("schema={schema}");

    let resp: HttpResponse = handle_http_request("POST", "/validate", &query, &[], body);
    assert_eq!(resp.status, 200);
    assert!(resp.body.contains("\"valid\":true"));
}

// spin-http-gateway AC-3: Invalid requests return HTTP 400 Bad Request with structured errors
#[test]
fn test_http_post_invalid_payload() {
    let schema = r#"{
        "type": "object",
        "properties": { "age": { "type": "integer", "minimum": 0 } },
        "required": ["age"]
    }"#;
    let body = br#"{"age": -10}"#;
    let headers = vec![("X-Schema-ID".to_string(), schema.to_string())];

    let resp = handle_http_request("POST", "/validate", "", &headers, body);
    assert_eq!(resp.status, 400);
    assert!(resp.body.contains("\"valid\": false") || resp.body.contains("\"valid\":false"));
    assert!(resp.body.contains("/age"));
}

// spin-http-gateway AC-1: Reject missing schema parameter with HTTP 400
#[test]
fn test_http_missing_schema() {
    let body = br#"{"test": 1}"#;
    let resp = handle_http_request("POST", "/validate", "", &[], body);
    assert_eq!(resp.status, 400);
    assert!(resp.body.contains("Missing schema parameter"));
}

// spin-http-gateway AC-1: Reject invalid HTTP method with HTTP 405
#[test]
fn test_http_invalid_method() {
    let resp = handle_http_request("GET", "/validate", "", &[], &[]);
    assert_eq!(resp.status, 405);
}

// spin-http-gateway AC-4: CSV payload validation over HTTP using Content-Type header
#[test]
fn test_http_csv_payload() {
    let schema = r#"{
        "type": "array",
        "items": {
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "score": { "type": "integer" }
            },
            "required": ["name", "score"]
        }
    }"#;
    let body = b"name,score\nAlice,100\nBob,95\n";
    let query = format!("schema={schema}");
    let headers = vec![("Content-Type".to_string(), "text/csv".to_string())];

    let resp = handle_http_request("POST", "/validate", &query, &headers, body);
    assert_eq!(resp.status, 200);
    assert!(resp.body.contains("\"valid\":true"));
}
