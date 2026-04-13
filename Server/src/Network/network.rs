use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

pub fn send_data(stream: &mut TcpStream, buffer: &[u8]) {
    match stream.write_all(buffer) {
        Ok(_) => println!("Data sent successfully"),
        Err(e) => println!("Error sending data: {}", e),
    }
}
pub fn read_data(stream: &mut TcpStream) -> Option<Vec<u8>> {
    let mut buffer = [0u8; 4096];
    match stream.read(&mut buffer) {
        Ok(n) => {
            //println!("Data from server: {}", String::from_utf8_lossy(&buffer[..n]));
            Some(buffer[..n].to_vec())
        }
        Err(e) => {
            println!("Error reading from server: {}", e);
            None
        }
    }
}


