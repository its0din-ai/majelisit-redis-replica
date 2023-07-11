use serde_resp::{ de, RESPType };

pub fn dezer(req: &str) -> Vec<RESPType> {
    let mut hasil: Vec<RESPType> = Vec::new();
    let serialized_data: RESPType = de::from_str(&req).unwrap();
    // println!("{:?}", serialized_data);
    match serialized_data {
        RESPType::Array(bulk_data) => {
            if let Some(data) = bulk_data {
                for cmd in data {
                    match cmd {
                        RESPType::BulkString(bulk_data) => {
                            hasil.push(RESPType::BulkString(bulk_data));
                        }
                        _ => {
                            hasil.push(RESPType::Error("ERR: invalid command".to_string()));
                        }
                    }
                }
            }
        }
        _ => {
            hasil.push(RESPType::Error("ERR: invalid command".to_string()));
        }
    }
    return hasil;
}
