use std::{cmp::Reverse, collections::BinaryHeap};

struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = usize::try_from(k).unwrap();
        let mut kth_largest = Self {
            k,
            heap: BinaryHeap::with_capacity(k + 1),
        };
        nums.into_iter().for_each(|x| kth_largest.reorder_max(x));
        kth_largest
    }

    fn add(&mut self, val: i32) -> i32 {
        self.reorder_max(val);
        if let Some(Reverse(x)) = self.heap.peek() {
            return *x;
        }
        unreachable!();
    }

    fn reorder_max(&mut self, val: i32) {
        match self.heap.peek() {
            Some(Reverse(min)) => {
                if val > *min || self.heap.len() < self.k {
                    self.heap.push(Reverse(val));
                    if self.heap.len() > self.k {
                        self.heap.pop();
                    }
                }
            }
            _ => self.heap.push(Reverse(val)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut kth_largest = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(4, kth_largest.add(3));
        assert_eq!(5, kth_largest.add(5));
        assert_eq!(5, kth_largest.add(10));
        assert_eq!(8, kth_largest.add(9));
        assert_eq!(8, kth_largest.add(4));
    }
}
