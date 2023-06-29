use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use log::{LevelFilter, error, info};
use env_logger;

fn hndlrz(mut strim: TcpStream) {
    let mut buffz = [0; 1024];

    loop {
        match strim.read(&mut buffz) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                let reqz = String::from_utf8_lossy(&buffz[..n]).to_uppercase();
                let resz = if reqz.trim() == "PING" {
                    "PONG\r\n".to_owned()
                } else {
                    format!("DEBUG => {}\r", reqz)
                };

                strim.write_all(resz.as_bytes()).unwrap();
            }
            Err(e) => {
                error!("External Error: {}", e);
                break;
            }
        }

        buffz = [0; 1024];
    }
}

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .init();

    let liztnr = TcpListener::bind("0.0.0.0:6379").expect("Bind failed bang, port udh kepake?");

    info!("Listening. . .");

    for strim in liztnr.incoming() {
        match strim {
            Ok(strim) => {
                thread::spawn(move || {
                    hndlrz(strim);
                });
            }
            Err(e) => {
                error!("Internal Error: {}", e);
            }
        }
    }
}