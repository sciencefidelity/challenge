#![allow(
    clippy::similar_names,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation
)]
pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut lis_len, mut lds_len) = (vec![1; n], vec![1; n]);
        for i in 1..n {
            for j in (0..i).rev() {
                if nums[i] > nums[j] {
                    lis_len[i] = lis_len[i].max(lis_len[j] + 1);
                }
            }
        }
        for i in (0..n).rev() {
            for j in i + 1..n {
                if nums[i] > nums[j] {
                    lds_len[i] = lds_len[i].max(lds_len[j] + 1);
                }
            }
        }
        let mut min_removals = i32::MAX;
        for i in 0..n {
            if lis_len[i] > 1 && lds_len[i] > 1 {
                min_removals = min_removals.min((n + 1 - lis_len[i] - lds_len[i]) as i32);
            }
        }
        min_removals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::minimum_mountain_removals(vec![1, 3, 1]), 0);
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::minimum_mountain_removals(vec![2, 1, 1, 5, 6, 2, 3, 1]),
            3
        );
    }
}
