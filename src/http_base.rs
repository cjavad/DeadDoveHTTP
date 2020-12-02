use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub struct HTTPRequest {
    pub method: String,
    pub path: String,
    pub protocol: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: String
}

pub struct HTTPResponse {
    pub status_code: i32,
    pub status_text: String,
    pub protocol: String,
    pub headers: HashMap<String, String>,
    pub body: String
}

lazy_static! {
    static ref RE_HTTP_HEADER_GLOBAL: Regex = Regex::new(r"([\w\-]+):\s(.+)").unwrap();
    static ref RE_HTTP_REQUEST_GLOBAL: Regex = Regex::new(r"(GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH)\s(/(.+)?)\s(HTTP/[0-9\.]{1,3})").unwrap();
    static ref RE_URL_QUERY_STRING_GLOBAL: Regex = Regex::new(r"[?&]([^&]+)=([^&]+)").unwrap();
}

pub fn is_header(text: &str) -> bool {
    return RE_HTTP_HEADER_GLOBAL.is_match(text);
}

pub fn is_request_line(text: &str) -> bool {
    return RE_HTTP_REQUEST_GLOBAL.is_match(text);
}

pub fn parse_http_request_line(http_request_line: &str) -> (String, String, String) {
    // Capture regex groups
    let caps = RE_HTTP_REQUEST_GLOBAL.captures(http_request_line).unwrap();
    // Return groups (1: method, 2: path and 4: protocol)
    return (caps.get(1).map_or("".to_string(), |m| m.as_str().to_string()), caps.get(2).map_or("".to_string(), |m| m.as_str().to_string()), caps.get(4).map_or("".to_string(), |m| m.as_str().to_string()));
}

pub fn parse_http_request_header(header: &str) -> (String, String) {
    // Capture regex groups
    let caps = RE_HTTP_HEADER_GLOBAL.captures(header).unwrap();
    // Return groups (1: Key, 2: Value)
    return (caps.get(1).map_or("".to_string(), |m| m.as_str().to_string()), caps.get(2).map_or("".to_string(), |m| m.as_str().to_string()));
}

pub fn parse_url_query_string(path: String) -> (String, HashMap<String, String>) {
    let mut query_params: HashMap<String, String> = HashMap::new();
    let mut index: usize = path.len();

    for (i, c) in path.chars().enumerate() {
        if c == '?' {
            // Include
            index = i;
            break;
        }
    }

    let remaining_path: String = path[0..index].to_string();
    let query_string: String = path[index..path.len()].to_string();

    let query_strings: Vec<String> = RE_URL_QUERY_STRING_GLOBAL.find_iter(query_string.as_str()).map(|m| m.as_str().to_string()).collect();

    for qs in query_strings {
        let caps = RE_URL_QUERY_STRING_GLOBAL.captures(qs.as_str()).unwrap();
        query_params.insert(caps.get(1).map_or("".to_string(), |m| m.as_str().to_string()), caps.get(1).map_or("".to_string(), |m| m.as_str().to_string()));
    }

    return (remaining_path, query_params);
}

pub fn create_http_response(response: HTTPResponse) -> String {
    let mut response_string = String::new();
    let mut headers = response.headers;

    response_string += format!("{} {} {}\r\n", response.protocol, response.status_code, response.status_text).as_str();
    
    for (key, value) in &* &mut headers {
        response_string += format!("{}: {}\r\n", key, value).as_str();
    }

    response_string += format!("\r\n{}", response.body).as_str();
    return response_string;
}