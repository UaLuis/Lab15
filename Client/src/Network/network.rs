use std::io::{Read, Write};
use std::net::TcpStream;

pub fn send_data(stream: &mut TcpStream, msg: String) {
    stream.write(msg.as_bytes()).expect("Error send data to server");
}

pub fn read_data(stream: &mut TcpStream) {
    let mut buffer = [0, 1024];
    match stream.read(&mut buffer) {
        Ok(n) => {
            println!("Data from server: {}", String::from_utf8_lossy(&buffer[..n]));
        }
        Err(e) => {
            println!("Error reading from server: {}", e);
        }
    }
}