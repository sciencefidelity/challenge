use std::cmp::min;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let (row, col) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; col + 1]; row + 1];
        let mut result = 0;
        for i in 0..row {
            for j in 0..col {
                if matrix[i][j] == 1 {
                    dp[i + 1][j + 1] = min(min(dp[i][j + 1], dp[i + 1][j]), dp[i][j]) + 1;
                    result += dp[i + 1][j + 1];
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::count_squares(arr![[0, 1, 1, 1], [1, 1, 1, 1], [0, 1, 1, 1]]),
            15
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::count_squares(arr![[1, 0, 1], [1, 1, 0], [1, 1, 0]]),
            7
        );
    }
}
