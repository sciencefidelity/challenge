use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut min_heap = BinaryHeap::with_capacity(nums.len());
        for (i, n) in nums.iter().enumerate() {
            min_heap.push(Reverse((*n, i)));
        }
        for _ in 0..k {
            let candidate = min_heap.pop().unwrap().0;
            nums[candidate.1] *= multiplier;
            min_heap.push(Reverse((candidate.0 * multiplier, candidate.1)));
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::get_final_state(vec![2, 1, 3, 5, 6], 5, 2),
            vec![8, 4, 6, 5, 6]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::get_final_state(vec![1, 2], 3, 4), vec![16, 8]);
    }
}
