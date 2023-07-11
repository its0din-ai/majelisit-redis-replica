use std::collections::HashMap;


pub struct DataStore {
    data: HashMap<String, String>,
}

impl DataStore {
    pub fn new() -> DataStore {
        DataStore {
            data: HashMap::new(),
        }
    }
    
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }
    
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn del(&mut self, key: String) -> Option<String> {
        self.data.remove(&key)
    }
}