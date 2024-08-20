pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = usize::try_from(amount).unwrap();
        let mut dp = vec![None; amount + 1];
        dp[0] = Some(0);
        for i in 1..=amount {
            dp[i] = coins
                .iter()
                .filter_map(|&j| {
                    let j = usize::try_from(j).unwrap();
                    if j <= i {
                        dp[i - j].map(|n| n + 1)
                    } else {
                        None
                    }
                })
                .min();
        }
        dp[amount].unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
    }
}
