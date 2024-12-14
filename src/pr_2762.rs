#![allow(unused_assignments, clippy::cast_possible_wrap)]
pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let (mut right, mut left) = (0, 0);
        let (mut cur_min, mut cur_max) = (nums[right], nums[right]);
        let (mut window_len, mut total) = (0, 0_i64);
        while right < nums.len() {
            cur_min = cur_min.min(nums[right]);
            cur_max = cur_max.max(nums[right]);
            if cur_max - cur_min > 2 {
                window_len = right - left;
                total += (window_len * (window_len + 1) / 2) as i64;
                left = right;
                (cur_min, cur_max) = (nums[right], nums[right]);
                while left > 0 && (nums[right] - nums[left - 1]).abs() <= 2 {
                    left -= 1;
                    (cur_min, cur_max) = (cur_min.min(nums[left]), cur_max.max(nums[left]));
                }
                if left < right {
                    window_len = right - left;
                    total -= (window_len * (window_len + 1) / 2) as i64;
                }
            }
            right += 1;
        }
        window_len = right - left;
        total += (window_len * (window_len + 1) / 2) as i64;
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::continuous_subarrays(vec![5, 4, 2, 4]), 8);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::continuous_subarrays(vec![1, 2, 3]), 6);
    }
}
