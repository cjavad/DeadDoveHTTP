use std::collections::HashMap;
use crate::http_base;

pub fn req(request: http_base::HTTPRequest) -> http_base::HTTPResponse {
    let mut status_code = 200;
    let mut status_text = "OK".to_string();
    let protocol = request.protocol;
    let mut headers: HashMap<String, String> = HashMap::new();
    let mut body: String;

    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => { headers.insert("Content-Type".to_string(), "text/html".to_string()); body = "<h1>Hello World</h1>".to_string(); },
        _ => { status_code = 400; status_text = "Not found".to_string(); headers.insert("Content-Type".to_string(), "text/html".to_string()); body = "<h1>404: Not found</h1>".to_string(); }
    }

    return http_base::HTTPResponse {
        status_code: status_code,
        status_text: status_text,
        protocol: protocol,
        headers: headers,
        body: body
    }
}
