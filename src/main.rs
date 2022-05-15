use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
fn main() {
    let listener = TcpListener::bind("0.0.0.0:8081").unwrap();
    println!("Server is Running on Port 8081");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => println!("Unable to read stream: {}", e),
        }
    }
}
fn handle_read(mut stream: &TcpStream) -> String {
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            println!("{}", req_str);

            let mut request = req_str.split_whitespace();
            request.next();
            let path = request.next().unwrap();
            println!("{}", path);
            return path.to_lowercase();
        }
        Err(e) => println!("Unable to read stream: {}", e),
    }
    return "404".to_string();
}
fn handle_write(mut stream: TcpStream, path: &str) {
    let filename;
    if path == "/" || path == "404" {
        filename = "index.html";
    } else {
        filename = path
    }
    let response = read_file(filename);
    println!("{:?}", response);
    let body = match response {
        Some(b) => b,
        None => "".to_string(),
    };
    write!(stream, "HTTP/1.1 {} {}\r\n\r\n{}", 200, "Ok", body).ok();
    stream.flush().unwrap()
}

fn handle_client(stream: TcpStream) {
    let path = handle_read(&stream);
    handle_write(stream, &path);
}
fn get_current_directory() -> String {
    let cwd = env::current_dir().unwrap();
    return cwd.into_os_string().into_string().unwrap();
}
fn read_file(file_path: &str) -> Option<String> {
    let default_path = format!("{}/public", get_current_directory());
    let path = format!("{}/{}", default_path, file_path);
    println!("Read File: {}", path);
    match fs::canonicalize(path) {
        Ok(path) => {
            if path.starts_with(&default_path) {
                let contents = fs::read_to_string(path).ok();
                return contents;
            } else {
                println!("Directory Traversal Attack Attempted: {}", file_path);
                None
            }
        }
        Err(_) => None,
    }
}
