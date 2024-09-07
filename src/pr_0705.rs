const LEN: usize = 1_000_001;

struct MyHashSet {
    buf: Box<[bool]>,
}

impl MyHashSet {
    fn new() -> Self {
        Self {
            buf: vec![false; LEN].into_boxed_slice(),
        }
    }

    fn add(&mut self, key: i32) {
        self.buf[usize::try_from(key).unwrap()] = true;
    }

    fn remove(&mut self, key: i32) {
        self.buf[usize::try_from(key).unwrap()] = false;
    }

    fn contains(&self, key: i32) -> bool {
        self.buf[usize::try_from(key).unwrap()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut my_hash_set = MyHashSet::new();
        my_hash_set.add(1);
        my_hash_set.add(2);
        assert!(my_hash_set.contains(1));
        assert!(!my_hash_set.contains(3));
        my_hash_set.add(2);
        assert!(my_hash_set.contains(2));
        my_hash_set.remove(2);
        assert!(!my_hash_set.contains(2));
    }
}
