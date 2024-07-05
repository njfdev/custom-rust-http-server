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

    let status_code: &str;

    match request_endpoint {
        "/" => {
            status_code = "200 OK";
        }
        _ => {
            status_code = "404 Not Found";
        }
    }

    stream.write(format!("HTTP/1.1 {}\r\n\r\n", status_code).as_bytes()).unwrap();
    stream.flush().unwrap();
}