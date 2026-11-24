use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use wasm_json_validator::provider::http::handle_http_request;

#[http_component]
fn handle_validator(req: Request) -> anyhow::Result<impl IntoResponse> {
    let method = req.method().to_string();
    let uri = req.uri();
    let (path, query) = match uri.split_once('?') {
        Some((p, q)) => (p, q),
        None => (uri, ""),
    };

    let mut headers = Vec::new();
    for (k, v) in req.headers() {
        if let Ok(val_str) = std::str::from_utf8(v.as_bytes()) {
            headers.push((k.to_string(), val_str.to_string()));
        }
    }

    let body = req.body();
    let resp = handle_http_request(&method, path, query, &headers, body);

    let mut response = Response::new(resp.status, resp.body);
    for (k, v) in resp.headers {
        response.set_header(k, v);
    }
    Ok(response)
}
