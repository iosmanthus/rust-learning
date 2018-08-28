use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    let mut buf = BufReader::new(stream);
    buf.get_mut().write_all(b"hello world\r\n").unwrap();
    buf.get_mut().flush().unwrap();

    for stream in listener.incoming() {
        let mut stream = BufReader::new(stream.unwrap());
        let mut msg = String::new();
        stream.read_line(&mut msg).unwrap();
        msg = msg.trim().to_string();
        println!("{}", msg);
    }
}
