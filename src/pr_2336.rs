use std::collections::{BinaryHeap, HashSet};

struct SmallestInfiniteSet {
    heap: BinaryHeap<i32>,
    set: HashSet<i32>,
    smallest: i32,
}

impl SmallestInfiniteSet {
    fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            set: HashSet::new(),
            smallest: 1,
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        if let Some(n) = self.heap.pop() {
            self.set.remove(&n);
            -n
        } else {
            self.smallest += 1;
            self.smallest - 1
        }
    }

    fn add_back(&mut self, num: i32) {
        if num >= self.smallest {
            return;
        }
        if !self.set.contains(&(-num)) {
            self.heap.push(-num);
            self.set.insert(-num);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut isi = SmallestInfiniteSet::new();
        isi.add_back(2);
        assert_eq!(isi.pop_smallest(), 1);
        assert_eq!(isi.pop_smallest(), 2);
        assert_eq!(isi.pop_smallest(), 3);
        isi.add_back(1);
        assert_eq!(isi.pop_smallest(), 1);
        assert_eq!(isi.pop_smallest(), 4);
        assert_eq!(isi.pop_smallest(), 5);
    }
}
