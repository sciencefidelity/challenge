#![allow(clippy::cast_sign_loss)]
pub struct Solution;

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut dp = vec![0; 1 << 17];
        dp[0] = 1;
        for num in nums {
            for i in (0..=max).rev() {
                dp[(i | num) as usize] += dp[i as usize];
            }
            max |= num;
        }
        dp[max as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::count_max_or_subsets(vec![3, 1]), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::count_max_or_subsets(vec![2, 2, 2]), 7);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::count_max_or_subsets(vec![3, 2, 1, 5]), 6);
    }
}
