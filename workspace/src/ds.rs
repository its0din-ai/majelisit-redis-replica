use std::collections::HashMap;
use std::time::{ Duration, Instant };

pub struct DataStore {
    data: HashMap<String, String>,
    exp_time: HashMap<String, Option<Instant>>,
}

impl DataStore {
    pub fn new() -> DataStore {
        DataStore {
            data: HashMap::new(),
            exp_time: HashMap::new(),
        }
    }

    pub async fn set(&mut self, key: String, value: String, exp_time: Option<Duration>) {
        let now = Instant::now();
        self.data.insert(key.clone(), value);
        match exp_time {
            Some(time) => {
                self.exp_time.insert(key, Some(now + time));
            }
            None => {
                self.exp_time.insert(key, None);
            }
        }
    }

    pub async fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub async fn del(&mut self, key: String) -> Option<String> {
        self.data.remove(&key)
    }

    pub async fn remove_expired(&mut self) {
        let now = Instant::now();
        self.exp_time.retain(|_, expiration| expiration.map(|exp| exp > now).unwrap_or(true));

        let expired_keys: Vec<_> = self.data
            .keys()
            .filter(|key| !self.exp_time.contains_key(*key))
            .cloned()
            .collect();
        
        for key in expired_keys {
            self.data.remove(&key);
        }
    }
}