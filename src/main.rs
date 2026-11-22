use std::collections::{HashMap, LinkedList};


pub struct LruCache {
    pub capacity: usize,
    pub hashmap: HashMap<i32, i32>,
    pub order_list: LinkedList<i32>
}

impl LruCache {
    pub fn new(cap: usize) -> Self {
        if cap == 0 {
            panic!("Can not make a LruCache with capacity 0");
        }
        
        let lru = LruCache {
            capacity: cap,
            hashmap: HashMap::with_capacity(cap),
            order_list: LinkedList::new()
        };

        return lru;
    }

    pub fn put (&mut self, key: i32, val: i32) {
        let len = self.hashmap.len();

        if len == self.capacity {
            
            if let Some(last_key) = self.order_list.back() {
                self.hashmap.remove(last_key);
            }

            self.order_list.push_front(key);
            self.hashmap.insert(key, val);
            
        }

        else {
            self.order_list.push_front(key);
            self.hashmap.insert(key, val);
        }
    }

    pub fn get(&self, key: i32) -> Option<&i32>{
        self.hashmap.get(&key)
    }
    
}

fn main() {
}

#[cfg(test)]
mod tests;
