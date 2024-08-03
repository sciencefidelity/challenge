use std::cmp::min;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let mut miniumu_swaps = i32::MAX;
        let total_ones: i32 = nums.iter().sum();
        let mut ones_count = nums[0];
        let mut end = 0;
        for start in 0..nums.len() {
            if start != 0 {
                ones_count -= nums[start - 1];
            }
            while end - start + 1 < total_ones.try_into().unwrap() {
                end += 1;
                ones_count += nums[end % nums.len()];
            }
            miniumu_swaps = min(miniumu_swaps, total_ones - ones_count);
        }
        miniumu_swaps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(1, Solution::min_swaps(vec![0, 1, 0, 1, 1, 0, 0]));
    }

    #[test]
    fn case_2() {
        assert_eq!(2, Solution::min_swaps(vec![0, 1, 1, 1, 0, 0, 1, 1, 0]));
    }

    #[test]
    fn case_3() {
        assert_eq!(0, Solution::min_swaps(vec![1, 1, 0, 0, 1]));
    }
}
