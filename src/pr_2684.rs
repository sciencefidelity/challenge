#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut memo = HashMap::with_capacity(rows * cols);
        let mut max_moves = 0;
        for row in 0..rows {
            max_moves = max_moves.max(Self::dfs(row as i32, 0, &grid, &mut memo));
        }
        max_moves
    }

    fn dfs(row: i32, col: i32, grid: &[Vec<i32>], memo: &mut HashMap<(i32, i32), i32>) -> i32 {
        if let Some(&moves) = memo.get(&(row, col)) {
            return moves;
        }

        let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
        let mut moves = 0;

        for (r, c) in [(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)] {
            if r >= 0
                && r < rows
                && c >= 0
                && c < cols
                && grid[r as usize][c as usize] > grid[row as usize][col as usize]
            {
                moves = moves.max(1 + Self::dfs(r, c, grid, memo));
            }
        }
        memo.insert((row, col), moves);
        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        let grid = arr![[2, 4, 3, 5], [5, 4, 9, 3], [3, 4, 2, 11], [10, 9, 13, 15]];
        assert_eq!(Solution::max_moves(grid), 3);
    }

    #[test]
    fn case_2() {
        let grid = arr![[3, 2, 4], [2, 1, 9], [1, 1, 7]];
        assert_eq!(Solution::max_moves(grid), 0);
    }
}
