#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut min_length = i32::MAX;
        let (mut window_start, mut window_end) = (0, 0);
        let mut bit_counts = vec![0; 32];
        while window_end < nums.len() as i32 {
            Self::update_bit_counts(&mut bit_counts, nums[window_end as usize], 1);
            while window_start <= window_end && Self::convert_bit_counts_to_number(&bit_counts) >= k
            {
                min_length = min_length.min(window_end - window_start + 1);
                Self::update_bit_counts(&mut bit_counts, nums[window_start as usize], -1);
                window_start += 1;
            }
            window_end += 1;
        }
        if min_length == i32::MAX {
            -1
        } else {
            min_length
        }
    }

    fn update_bit_counts(bit_counts: &mut [i32], number: i32, delta: i32) {
        for (position, count) in bit_counts.iter_mut().enumerate() {
            if number & (1 << position) != 0 {
                *count += delta;
            }
        }
    }

    fn convert_bit_counts_to_number(bit_counts: &[i32]) -> i32 {
        let mut result = 0;
        for (position, count) in bit_counts.iter().enumerate() {
            if *count != 0 {
                result |= 1 << position;
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
        assert_eq!(Solution::minimum_subarray_length(vec![1, 2, 3], 2), 1);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::minimum_subarray_length(vec![2, 1, 8], 10), 3);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::minimum_subarray_length(vec![1, 2], 0), 1);
    }
}
