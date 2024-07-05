use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                handle_request(&mut _stream);
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

    let mut status_code: &str = "200 OK";
    let mut headers: Vec<String> = Vec::new();
    let mut body: String = "".to_string();

    if request_endpoint == "/" {
        
    } else if request_endpoint.to_string().starts_with("/echo/") {
        body = request_endpoint.to_string().replace("/echo/", "");

        headers.push("Content-Type: text/plain".to_string());
        headers.push(format!("Content-Length: {}", body.len()));
    } else {
        status_code = "404 Not Found";
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