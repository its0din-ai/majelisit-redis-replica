use std::io::{ Read, Write };
use std::net::TcpListener;
mod ds;
mod serdeser;
use ds::DataStore;
use serdeser::dezer;
use serde_resp::RESPType;

#[allow(unused_assignments)]
#[tokio::main]
async fn main() {
    let mut data_stor = DataStore::new();
    let liztnr = TcpListener::bind("0.0.0.0:6379").expect("Bind failed bang, port udh kepake?");
    println!("Lizten di port 6379 bang!");

    loop {
        match liztnr.accept() {
            Ok((mut strim, _)) => {
                let mut buffer = [0; 512];
                match strim.read(&mut buffer) {
                    Ok(_) => {
                        let req = String::from_utf8_lossy(&buffer[..]);
                        let hasil = dezer(&req);
                        let mut command: Vec<String> = Vec::new();

                        for cmd in hasil {
                            match cmd {
                                RESPType::BulkString(bulk_data) => {
                                    for fin in &bulk_data {
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
                        if command.len() > 0 {
                            let mut resp: &[u8] = b"";
                            if command[0] == "PING" {
                                resp = b"+PONG\r\n";
                                strim.write(resp.clone()).unwrap();
                            } else if command[0] == "SET" {
                                if command.len() <= 3{
                                    data_stor.set(command[1].clone(), command[2].clone()).await;
                                    resp = b"+OK\r\n";
                                    strim.write(resp.clone()).unwrap();
                                    println!("SET OK: {}", command[1].clone());
                                } else {


                                    
                                    // ["SET", "foo2", "barrrrrrr", "EX", "3", "NX"]
                                    let time = command[4].parse::<u64>().unwrap();
                                    data_stor.set(command[1].clone(), command[2].clone()).await;
                                    resp = b"+OK\r\n";
                                    strim.write(resp.clone()).unwrap();
                                    tokio::time::sleep(tokio::time::Duration::from_secs(time.into())).await;
                                    data_stor.del(command[1].clone()).await;




                                }
                            } else if command[0] == "GET" {
                                let hasil = data_stor.get(&command[1].clone()).await;
                                match hasil {
                                    Some(hasil) => {
                                        let fmt = format!{"${}\r\n{}\r\n", hasil.len(), hasil};
                                        println!("hasil:: {}", fmt);
                                        let rsp = fmt.as_bytes();
                                        strim.write(rsp).unwrap();
                                        println!("GET OK: {}", command[1].clone());
                                    }
                                    None => {
                                        resp = b"$-1\r\n";
                                        strim.write(resp.clone()).unwrap();
                                        println!("GET ERR: {}", command[1].clone());
                                    }
                                }
                            } else if command[0] == "DEL" {
                                data_stor.del(command[1].clone()).await;
                                resp = b"$1\r\n1\r\n";
                                strim.write(resp.clone()).unwrap();
                                println!("DEL OK: {}", command[1].clone());
                            } else {
                                resp = b"-ERR unknown command\r\n";
                                strim.write(resp.clone()).unwrap();
                            }
                            // println!("{:?}", resp);
                        } else {
                            let resp: &[u8] = b"KOSONG\r\n";
                            strim.write(resp).unwrap();
                        }
                    }
                    Err(e) => {
                        println!("ERRRRR: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("ERRRRR: {}", e);
            }
        }
        // print!("Connection closed");/
    }
}
