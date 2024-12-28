#![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum = nums.iter().sum::<i32>();
        let mut dp = vec![vec![0; (2 * sum + 1) as usize]; nums.len()];
        dp[0][(nums[0] + sum) as usize] = 1;
        dp[0][(-nums[0] + sum) as usize] += 1;
        for i in 1..nums.len() {
            for cur in -sum..=sum {
                if dp[i - 1][(cur + sum) as usize] > 0 {
                    dp[i][(cur + nums[i] + sum) as usize] += dp[i - 1][(cur + sum) as usize];
                    dp[i][(cur - nums[i] + sum) as usize] += dp[i - 1][(cur + sum) as usize];
                }
            }
        }
        if target.abs() > sum {
            0
        } else {
            dp[nums.len() - 1][(target + sum) as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::find_target_sum_ways(vec![1], 1), 1);
    }
}
