#![allow(
    clippy::needless_pass_by_value,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_lossless
)]
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut ans = 0_i64;
        let mut current_sum = 0_i64;
        let mut begin = 0_i32;
        let mut end = 0_i32;
        let mut num_to_index: HashMap<i32, i32> = HashMap::new();
        while end < nums.len() as i32 {
            let curr_num = nums[end as usize];
            let last_occurrence = if num_to_index.contains_key(&curr_num) {
                num_to_index[&curr_num]
            } else {
                -1
            };
            while begin <= last_occurrence || end - begin + 1 > k {
                current_sum -= nums[begin as usize] as i64;
                begin += 1;
            }
            num_to_index.insert(curr_num, end);
            current_sum += nums[end as usize] as i64;
            if end - begin + 1 == k {
                ans = ans.max(current_sum);
            }
            end += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::maximum_subarray_sum(vec![1, 5, 4, 2, 9, 9, 9], 3),
            15
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::maximum_subarray_sum(vec![4, 4, 4], 3), 0);
    }
}
