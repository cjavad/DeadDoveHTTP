mod http_base;
use std::collections::HashMap;
use async_std::{
    net::{TcpListener, ToSocketAddrs, TcpStream},
    prelude::*,
    task,
    io::{BufReader, BufWriter}
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        println!("Accepting from: {}", stream.peer_addr()?);
        let handle = task::spawn(connection_handler(stream));
        let conn = handle.await;
    }

    Ok(())
}

async fn connection_handler(stream: TcpStream) -> Result<()> {
    let mut count: usize =  0;
    let stream_clone = stream.clone();
    let mut writer = BufWriter::new(stream);
    let reader = BufReader::new(stream_clone); // 2
    let mut lines = reader.lines();
    let mut still_headers = true;
    let mut method = String::new();
    let mut path = String::new();
    let mut protocol = String::new();
    let mut headers: HashMap<String, String> = HashMap::new();
    let mut body = String::new();

    while let Some(line) = lines.next().await {
        let line = line?;

        if line.is_empty() {
            // End of request
            break;
        }

        if count == 0 && http_base::is_request_line(line.as_str()) {
            let (m, p, proto) = http_base::parse_http_request_line(line.as_str());
            method = m;
            path = p;
            protocol = proto;
        } else if still_headers && http_base::is_header(line.as_str()) {
            let (k, v) = http_base::parse_http_request_header(line.as_str());
            headers.insert(k, v);
        } else if still_headers && !http_base::is_header(line.as_str()) {
            still_headers = false;
        } else {
            body += line.as_str();
        }

        count += 1;
    }

    let parsed_request = http_base::HTTPRequest {
        method: method,
        path: path,
        protocol: protocol,
        headers: headers,
        body: body
    };

    let mut response_headers: HashMap<String, String> = HashMap::new();
    response_headers.insert("Content-Type".to_string(), "text/html".to_string());

    let response = http_base::HTTPResponse {
        status_code: 200,
        status_text: "OK".to_string(),
        protocol: parsed_request.protocol,
        headers: response_headers,
        body: "<h3>FUCK OFF</h3>".to_string()
    };

    let response_string = http_base::create_http_response(response);

    writer.write(response_string.as_bytes()).await.unwrap();
    writer.flush().await.unwrap();
    Ok(())
}

fn run() {
    let fut = accept_loop("127.0.0.1:8080");
    task::block_on(fut);
}

fn main() -> Result<()> {
    run();
    Ok(())
}