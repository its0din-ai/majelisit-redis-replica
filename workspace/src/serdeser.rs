use serde_resp::{de, RESPType};

fn is_resp_serialized(value: &str) -> bool {
    let mut is_serialized = false;
    if value.starts_with('*') {
        is_serialized = true;
    }
    return is_serialized;
}

pub fn dezer(req: &str) -> Vec<RESPType> {
    // check if req is serialized
    if !is_resp_serialized(req) {
        return vec![RESPType::Error("ERR: invalid command".to_string())];
    }
    else{

        let mut hasil: Vec<RESPType> = Vec::new();
        let serialized_data: RESPType = de::from_str(&req).unwrap();
        // println!("serialized_data: {:?}", serialized_data);

        // handle panic
        match serialized_data {
            RESPType::Array(bulk_data) => {
                for data in bulk_data {
                    for cmd in data{
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
}


// RESPType::Array(bulk_data) => {
//     for data in bulk_data {
//         for cmd in data{
//             match cmd {
//                 RESPType::BulkString(bulk_data) => {
//                     hasil.push(RESPType::BulkString(bulk_data));
//                 }
//                 _ => {
//                     hasil.push(RESPType::Error("ERR: invalid command".to_string()));
//                 }
//             }
//         }
//     }
// }