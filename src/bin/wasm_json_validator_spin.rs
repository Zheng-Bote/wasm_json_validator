use std::io::{Read, Write};
use wasm_json_validator::provider::http::handle_http_request;

fn main() {
    let method = std::env::var("REQUEST_METHOD").unwrap_or_else(|_| "POST".into());
    let path = std::env::var("PATH_INFO").unwrap_or_else(|_| "/validate".into());
    let query = std::env::var("QUERY_STRING").unwrap_or_default();

    let mut headers = Vec::new();
    if let Ok(content_type) = std::env::var("CONTENT_TYPE") {
        headers.push(("Content-Type".into(), content_type));
    }
    if let Ok(schema_id) = std::env::var("HTTP_X_SCHEMA_ID") {
        headers.push(("X-Schema-ID".into(), schema_id));
    }

    let mut body = Vec::new();
    let _ = std::io::stdin().read_to_end(&mut body);

    let response = handle_http_request(&method, &path, &query, &headers, &body);

    let status_line = match response.status {
        200 => "200 OK",
        400 => "400 Bad Request",
        405 => "405 Method Not Allowed",
        _ => "500 Internal Server Error",
    };

    print!("Status: {status_line}\r\n");
    for (k, v) in &response.headers {
        print!("{k}: {v}\r\n");
    }
    print!("\r\n");
    print!("{}", response.body);
    let _ = std::io::stdout().flush();
}
