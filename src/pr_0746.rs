use std::cmp::min;

pub struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(costs: Vec<i32>) -> i32 {
        let mut dp = vec![0; costs.len() + 2];
        for (i, cost) in costs.into_iter().enumerate().rev() {
            dp[i] = cost + min(dp[i + 1], dp[i + 2]);
        }
        min(dp[0], dp[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(15, Solution::min_cost_climbing_stairs(vec![10, 15, 20]));
    }

    #[test]
    fn case_2() {
        assert_eq!(
            6,
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
        );
    }
}
