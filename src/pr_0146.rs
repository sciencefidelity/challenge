use std::collections::{HashMap, VecDeque};

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, i32>,
    queue: VecDeque<i32>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let capacity = usize::try_from(capacity).unwrap();
        Self {
            capacity,
            map: HashMap::with_capacity(capacity),
            queue: VecDeque::with_capacity(capacity),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.map.get(&key) {
            Some(val) => {
                if let Some(i) = self.queue.iter().position(|&x| x == key) {
                    self.queue.remove(i);
                    self.queue.push_front(key);
                }
                *val
            }
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            if let Some(i) = self.queue.iter().position(|&x| x == key) {
                self.queue.remove(i);
                self.queue.push_front(key);
            }
            self.map.insert(key, value);
        } else {
            if self.map.len() == self.capacity {
                let last = self.queue.pop_back().unwrap();
                self.map.remove(&last);
            }
            self.map.insert(key, value);
            self.queue.push_front(key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        assert_eq!(1, lru_cache.get(1));
        lru_cache.put(3, 3);
        assert_eq!(-1, lru_cache.get(2));
        lru_cache.put(4, 4);
        assert_eq!(-1, lru_cache.get(1));
        assert_eq!(3, lru_cache.get(3));
        assert_eq!(4, lru_cache.get(4));
    }
}
