use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread
};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                thread::spawn(move || {
                    handle_request(&mut _stream)
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


fn handle_request(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request: String = String::from_utf8_lossy(&buffer).to_string();
    let request_endpoint = request.split_whitespace().nth(1).unwrap();

    // request data
    let header = request.split("\r\n\r\n").nth(0).unwrap();
    let req_headers: Vec<String> = header.split("\r\n").skip(1).map(|s| s.to_string()).collect();
    let user_agent_header = req_headers.into_iter().filter(|s| s.to_ascii_lowercase().starts_with("user-agent: ")).nth(0);

    // response variables
    let mut status_code: &str = "200 OK";
    let mut content_type: &str = "text/plain";
    let mut headers: Vec<String> = Vec::new();
    let mut body: String = "".to_string();

    if request_endpoint == "/" {
        
    } else if request_endpoint.to_string().starts_with("/echo/") {
        body = request_endpoint.to_string().replace("/echo/", "");
    } else if request_endpoint == "/user-agent" {
        body = user_agent_header.expect("User Agent header exists").split(": ").nth(1).expect("User Agent has a value").to_string()
    } else {
        status_code = "404 Not Found";
    }

    // if there is a body, add required headers
    if body.len() > 0 {
        headers.push(format!("Content-Type: {}", content_type));
        headers.push(format!("Content-Length: {}", body.len()));
    }

    let mut response = format!("HTTP/1.1 {}\r\n", status_code);

    for header in headers {
        response.push_str(header.as_str());
        response.push_str("\r\n");
    }

    response.push_str("\r\n");
    response.push_str(body.as_str());

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}