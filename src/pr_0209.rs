pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut sum_of_current_window, mut left, mut right) = (0, 0, 0);
        let mut res = i32::MAX;
        while right < i32::try_from(nums.len()).unwrap() {
            sum_of_current_window += nums[usize::try_from(right).unwrap()];
            while sum_of_current_window >= target {
                res = res.min(right - left + 1);
                sum_of_current_window -= nums[usize::try_from(left).unwrap()];
                left += 1;
            }
            right += 1;
        }
        if res == i32::MAX {
            0
        } else {
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
            0
        );
    }
}
