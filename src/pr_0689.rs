#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation
)]
pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut best_single_start = 0;
        let mut best_double_start = [0, k];
        let mut best_triple_start = vec![0, k, k * 2];
        let mut current_window_sum_single = nums.iter().take(k as usize).sum::<i32>();
        let mut current_window_sum_double = nums
            .iter()
            .skip(k as usize)
            .take((k * 2) as usize)
            .sum::<i32>();
        let mut current_window_sum_triple = nums
            .iter()
            .skip((k * 2) as usize)
            .take((k * 3) as usize)
            .sum::<i32>();
        let mut best_single_sum = current_window_sum_single;
        let mut best_double_sum = current_window_sum_single + current_window_sum_double;
        let mut best_triple_sum =
            current_window_sum_single + current_window_sum_double + current_window_sum_triple;
        let mut single_start_index = 1;
        let mut double_start_index = k + 1;
        let mut triple_start_index = k * 2 + 1;
        while triple_start_index <= nums.len() as i32 - k {
            current_window_sum_single = current_window_sum_single - nums[single_start_index - 1]
                + nums[single_start_index + (k - 1) as usize];
            current_window_sum_double = current_window_sum_double
                - nums[(double_start_index - 1) as usize]
                + nums[(double_start_index + k - 1) as usize];
            current_window_sum_triple = current_window_sum_triple
                - nums[(triple_start_index - 1) as usize]
                + nums[(triple_start_index + k - 1) as usize];
            if current_window_sum_single > best_single_sum {
                best_single_start = single_start_index as i32;
                best_single_sum = current_window_sum_single;
            }
            if current_window_sum_double + best_single_sum > best_double_sum {
                best_double_start[0] = best_single_start;
                best_double_start[1] = double_start_index;
                best_double_sum = current_window_sum_double + best_single_sum;
            }
            if current_window_sum_triple + best_double_sum > best_triple_sum {
                best_triple_start[0] = best_double_start[0];
                best_triple_start[1] = best_double_start[1];
                best_triple_start[2] = triple_start_index;
                best_triple_sum = current_window_sum_triple + best_double_sum;
            }
            single_start_index += 1;
            double_start_index += 1;
            triple_start_index += 1;
        }
        best_triple_start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::max_sum_of_three_subarrays(vec![1, 2, 1, 2, 6, 7, 5, 1], 2),
            vec![0, 3, 5]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::max_sum_of_three_subarrays(vec![1, 2, 1, 2, 1, 2, 1, 2, 1], 2),
            vec![0, 2, 4]
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::max_sum_of_three_subarrays(vec![7, 13, 20, 19, 19, 2, 10, 1, 1, 19], 3),
            vec![1, 4, 7]
        );
    }
}
