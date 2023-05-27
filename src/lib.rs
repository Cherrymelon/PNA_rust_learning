use std::collections::HashMap;

pub struct KvStore {
    map: HashMap<String, String>,
}


impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new()
        }
    }
    pub fn get(&self, s: String) -> Option<String> {
        self.map.get(&s).cloned()
    }
    pub fn set(&mut self, s: String, v: String) -> Option<String> {
        self.map.insert(s, v)
    }
    pub fn remove(&mut self, s: String) -> Option<String> {
        self.map.remove(&s)
    }
}