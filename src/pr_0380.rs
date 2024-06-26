use rand::seq::SliceRandom;
use std::collections::HashMap;

pub struct Solution;

struct RandomizedSet {
    hash: HashMap<i32, usize>,
    v: Vec<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            hash: HashMap::new(),
            v: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.hash.contains_key(&val) {
            return false;
        }
        self.hash.insert(val, self.v.len());
        self.v.push(val);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        match self.hash.remove(&val) {
            None => false,
            Some(i) => {
                self.v.swap_remove(i);
                if i < self.v.len() {
                    self.hash.insert(self.v[i], i);
                }
                true
            }
        }
    }

    fn get_random(&self) -> i32 {
        *self.v.choose(&mut rand::thread_rng()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut set = RandomizedSet::new();

        assert!(set.insert(1));
        assert!(!set.remove(2));
        assert!(set.insert(2));

        set.get_random();

        assert!(set.remove(1));
        assert!(!set.insert(2));

        set.get_random();
    }
}
