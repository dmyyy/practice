// - if we want constant time get/put - need a doubly linked list
// - kinda stupid to do in rust
//
// TODO: do in golang later

use std::collections::{HashMap, LinkedList};

struct LRUCache {
    map: HashMap<i32, *mut usize>,
    values: Vec<i32>,
    capacity: usize,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let capacity: usize = capacity as usize;
        Self {
            map: HashMap::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
            capacity,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let idx_ptr = self.map.get(&key) else {
            return -1;
        };
        self.values[*idx_ptr]

        todo!()
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.map.len() == self.capacity as usize {
            self.map.remove(&key);
        }

        self.map.insert(key, value);
        self.recent_keys.push(key);
    }
}
