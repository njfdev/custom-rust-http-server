use std::{
    env, fs, io::{Read, Write}, net::{TcpListener, TcpStream}, thread
};

use itertools::Itertools;

fn main() {
    println!("Logs from your program will appear here!");

    let args = env::args().collect_vec();
    let files_directory_arg_index = args.iter().enumerate().find(|&(_, ref s)| s== &"--directory").map(|(index, _)| index);
    let mut files_directory: Option<String> = None;

    if files_directory_arg_index.is_some() {
        files_directory = Some(args[files_directory_arg_index.unwrap() + 1].clone());
    }

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let files_directory_clone = files_directory.clone();
                thread::spawn(move || {
                    handle_request(&mut _stream, files_directory_clone)
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


fn handle_request(stream: &mut TcpStream, files_directory: Option<String>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request: String = String::from_utf8_lossy(&buffer).to_string();
    let request_endpoint = request.split_whitespace().nth(1).expect("Request Endpoint in Request Header");
    let http_action = request.split_whitespace().nth(0).expect("HTTP Action in Request Header");

    // request data
    let header = request.split("\r\n\r\n").nth(0).unwrap();
    let req_headers: Vec<String> = header.split("\r\n").skip(1).map(|s| s.to_string()).collect();
    let user_agent_header = req_headers.clone().into_iter().filter(|s| s.to_ascii_lowercase().starts_with("user-agent: ")).nth(0);
    let request_body = request.split("\r\n\r\n").nth(1).unwrap();

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
    } else if request_endpoint.starts_with("/files/") {
        // get file path
        let file_path = request_endpoint.strip_prefix("/files").unwrap();

        let absolute_path = files_directory.expect("Files directory exists when requesting for a file") + file_path;

        if http_action == "GET" {

            // read the file
            let contents_result = fs::read_to_string(absolute_path);

            if contents_result.is_err() {
                status_code = "404 Not Found";
            } else {
                body = contents_result.unwrap();
                content_type = "application/octet-stream";
            }
        } else if http_action == "POST" {
            let content_length_header = req_headers.clone().into_iter().filter(|s| s.to_ascii_lowercase().starts_with("content-length: ")).exactly_one().expect("Content-Length header to exist");
            let content_length = content_length_header.split(": ").nth(1).unwrap().parse::<usize>().expect("Content-Length to be a number");

            let write_result = fs::write(absolute_path, request_body.chars().take(content_length).collect::<String>());

            if write_result.is_ok() {
                status_code = "201 Created";
            } else {
                status_code = "500 Internal Server Error";
            }
        }
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