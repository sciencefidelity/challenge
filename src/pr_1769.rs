#![allow(clippy::cast_possible_wrap)]
use std::iter::zip;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let nums: Vec<i32> = boxes
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .collect();
        let mut moves = 0;
        let mut count = 0;
        let n = nums.len();
        let mut left = vec![0; n];
        let mut right = vec![0; n];
        for i in 0..n {
            left[i] = moves;
            count += nums[i];
            moves += count;
        }
        moves = 0;
        count = 0;
        for i in (0..n).rev() {
            right[i] = moves;
            count += nums[i];
            moves += count;
        }
        zip(&left, &right).map(|(a, b)| a + b).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::min_operations("110".to_owned()), vec![1, 1, 3]);
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::min_operations("001011".to_owned()),
            vec![11, 8, 5, 4, 3, 4]
        );
    }
}
