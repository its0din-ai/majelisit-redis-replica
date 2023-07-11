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

    pub async fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }
    
    pub async fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub async fn del(&mut self, key: String) -> Option<String> {
        self.data.remove(&key)
    }
}
