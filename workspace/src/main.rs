use std::io::{Read, Write};
use std::net::{TcpListener};

fn main() {

    let liztnr = TcpListener::bind("0.0.0.0:6379").expect("Bind failed bang, port udh kepake?");
    println!("Lizten di port 6379 bang!");

    for strim in liztnr.incoming() {
        match strim {
            Ok(mut strim) => {
                let mut buf = [0; 512];
                strim.read(&mut buf).unwrap();
                strim.write("+PONG\r\n".as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}