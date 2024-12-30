pub struct Solution;

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        #[allow(clippy::cast_sign_loss)]
        let (low, high, zero, one) = (low as usize, high as usize, zero as usize, one as usize);
        let mut dp = vec![-1; high + 1];
        dp[0] = 1;
        let mut answer = 0;
        for end in low..=high {
            answer += Self::dfs(&mut dp, end, zero, one);
            answer %= MOD;
        }
        answer
    }

    fn dfs(dp: &mut Vec<i32>, end: usize, zero: usize, one: usize) -> i32 {
        if dp[end] != -1 {
            return dp[end];
        }
        let mut count = 0;
        if end >= one {
            count += Self::dfs(dp, end - one, zero, one);
        }
        if end >= zero {
            count += Self::dfs(dp, end - zero, zero, one);
        }
        dp[end] = count % MOD;
        dp[end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::count_good_strings(3, 3, 1, 1), 8);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::count_good_strings(2, 3, 1, 2), 5);
    }
}
