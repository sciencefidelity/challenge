use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let (mut score, mut marked, n) = (0, vec![false; nums.len()], nums.len());
        let mut heap = BinaryHeap::with_capacity(n);
        for (i, num) in nums.into_iter().enumerate() {
            heap.push(Reverse((num, i)));
        }
        while let Some(Reverse((num, i))) = heap.pop() {
            if !marked[i] {
                score += i64::from(num);
                marked[i] = true;
                if i > 0 {
                    marked[i - 1] = true;
                }
                if i < n - 1 {
                    marked[i + 1] = true;
                }
            }
        }
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::find_score(vec![2, 1, 3, 4, 5, 2]), 7);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::find_score(vec![2, 3, 5, 1, 3, 2]), 5);
    }
}
