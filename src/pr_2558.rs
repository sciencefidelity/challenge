#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut heap = BinaryHeap::from(gifts);
        for _ in 0..k {
            let gift = (heap.pop().unwrap() as f64).sqrt() as i32;
            heap.push(gift);
        }
        heap.into_iter().map(i64::from).sum::<i64>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::pick_gifts(vec![25, 64, 9, 4, 100], 4), 29);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::pick_gifts(vec![1, 1, 1, 1], 4), 4);
    }
}
