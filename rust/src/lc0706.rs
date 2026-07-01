use super::other::separate_chain_map::SeparateChainHashMap;

pub struct MyHashMap {
    map: SeparateChainHashMap<i32, i32>,
}

impl MyHashMap {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.map.put(key, value)
    }

    pub fn get(&self, key: i32) -> i32 {
        *self.map.get(&key).unwrap_or(&-1)
    }

    pub fn remove(&mut self, key: i32) {
        self.map.remove(&key);
    }
}
