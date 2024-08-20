use std::cmp::max;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        let (mut dp, mut suffix_sum) = (vec![vec![0; n + 1]; n + 1], vec![0; n + 1]);
        for (i, p) in piles.into_iter().enumerate().rev() {
            suffix_sum[i] = suffix_sum[i + 1] + p;
        }
        for i in 0..=n {
            dp[i][n] = suffix_sum[i];
        }
        for index in (0..n).rev() {
            for max_till_now in (1..n).rev() {
                let mut x = 1;
                while x <= 2 * max_till_now && index + x <= n {
                    dp[index][max_till_now] = max(
                        dp[index][max_till_now],
                        suffix_sum[index] - dp[index + x][max(max_till_now, x)],
                    );
                    x += 1;
                }
            }
        }
        dp[0][1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::stone_game_ii(vec![2, 7, 9, 4, 4]), 10);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 100]), 104);
    }
}
