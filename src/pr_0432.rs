use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};

#[derive(Eq, PartialEq)]
struct Key {
    k: String,
    v: i32,
}

impl Key {
    const fn new(k: String, v: i32) -> Self {
        Self { k, v }
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        let c = self.v.cmp(&other.v);
        if c == Ordering::Equal {
            return self.k.cmp(&other.k);
        }
        c
    }
}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct AllOne {
    store: HashMap<String, i32>,
    order: BTreeSet<Key>,
}

impl AllOne {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
            order: BTreeSet::new(),
        }
    }

    fn inc(&mut self, key: String) {
        if let Some(x) = self.store.get_mut(&key) {
            self.order.remove(&Key::new(key.clone(), *x));
            *x += 1;
            self.order.insert(Key::new(key, *x));
        } else {
            self.store.insert(key.clone(), 1);
            self.order.insert(Key::new(key.clone(), 1));
        }
    }

    fn dec(&mut self, key: String) {
        match self.store.get_mut(&key) {
            Some(1) => {
                self.store.remove(&key);
                self.order.remove(&Key::new(key.clone(), 1));
            }
            Some(x) => {
                self.order.remove(&Key::new(key.clone(), *x));
                *x -= 1;
                self.order.insert(Key::new(key, *x));
            }
            None => {}
        };
    }

    fn get_max_key(&self) -> String {
        self.order
            .last()
            .map_or_else(String::new, |key| key.k.clone())
    }

    fn get_min_key(&self) -> String {
        self.order
            .first()
            .map_or_else(String::new, |key| key.k.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut all_one = AllOne::new();
        all_one.inc("hello".to_owned());
        all_one.inc("hello".to_owned());
        assert_eq!(all_one.get_max_key(), "hello".to_owned());
        assert_eq!(all_one.get_min_key(), "hello".to_owned());
        all_one.inc("leet".to_owned());
        assert_eq!(all_one.get_max_key(), "hello".to_owned());
        assert_eq!(all_one.get_min_key(), "leet".to_owned());
    }
}
