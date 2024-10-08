#![allow(clippy::cast_sign_loss)]
use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let mut data: Vec<(i32, i32)> = nums2.into_iter().zip(nums1).collect();
        data.sort_unstable();
        let (mut pq, mut sum, mut result) = (BinaryHeap::with_capacity(data.len()), 0, 0);
        while let Some((a, b)) = data.pop() {
            sum += i64::from(b);
            pq.push(Reverse(b));
            if pq.len() > k as usize {
                if let Some(Reverse(c)) = pq.pop() {
                    sum -= i64::from(c);
                }
            }
            if pq.len() == k as usize {
                result = result.max(sum * i64::from(a));
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::max_score(vec![1, 3, 3, 2], vec![2, 1, 3, 4], 3),
            12
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::max_score(vec![4, 2, 3, 1, 1], vec![7, 5, 10, 9, 6], 1),
            30
        );
    }
}

/*
[3,1,3,2]
[1,2,3,4]
*/
