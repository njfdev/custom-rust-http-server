use std::net::TcpListener;
use std::io::Write;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                _stream.write(b"HTTP/1.1 200 OK\r\n\r\n").expect("write failed");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
