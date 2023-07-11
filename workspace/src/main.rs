use std::io::{ Read, Write };
use std::net::{ TcpListener, TcpStream };
mod ds; mod serdeser;
use ds::DataStore; use serdeser::dezer;
use serde_resp::{RESPType};

fn main() {
    let liztnr = TcpListener::bind("0.0.0.0:6379").expect("Bind failed bang, port udh kepake?");
    println!("Lizten di port 6379 bang!");
    
    // let mut data_stor = DataStore::new();

    for strim in liztnr.incoming() {
        match strim{
            Ok(mut strim) => {
                let mut buffer = [0; 512];
                match strim.read(&mut buffer){
                    Ok(_) => {
                        
                        let req = String::from_utf8_lossy(&buffer[..]);
                        let hasil = dezer(&req);
                        let mut command: Vec<String> = Vec::new();

                        for cmd in hasil{
                            match cmd{
                                RESPType::BulkString(bulk_data) => {
                                    
                                    for fin in &bulk_data{
                                        let command_part = String::from_utf8_lossy(&fin[..]);
                                        command.push(command_part.to_string());
                                    }

                                }
                                // print kosongan biar gk panic!
                                RESPType::SimpleString(_) => print!(""),
                                RESPType::Error(_) => print!(""),
                                RESPType::Integer(_) => print!(""),
                                RESPType::Array(_) => print!(""),
                            }
                            
                            
                        }
                        // APP LOGIC Disini banh
                        if command.len() > 0{
                            let mut resp: &[u8]= b"";
                            if command[0] == "PING"{
                                resp = b"+PONG\r\n";
                            }
                            strim.write(resp).unwrap();
                        }else{
                            let mut resp: &[u8]= b"KOSONG\r\n";
                            strim.write(resp).unwrap();
                        }



                       
                        
                    }
                    // TCPStream Error Handler
                    Err(e) => {
                        println!("ERRRRR: {}", e);
                    }
                }
            }
            Err(_) => print!(""),
        }

    }

    

}
