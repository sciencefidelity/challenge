#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
use std::collections::VecDeque;

pub struct Solution;

const DIRECTIONS: &[(i32, i32); 4] = &[(0, 1), (0, -1), (1, 0), (-1, 0)];

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        let (num_rows, num_cols, mut result) = (grid.len() as i32, grid[0].len() as i32, 0);
        let mut visited = vec![vec![false; num_cols as usize]; num_rows as usize];
        for i in 0..num_rows {
            for j in 0..num_cols {
                if grid[i as usize][j as usize] != 0 && !visited[i as usize][j as usize] {
                    result = result.max(Self::count_fishes(&grid, &mut visited, i, j));
                }
            }
        }
        result
    }

    fn count_fishes(grid: &[Vec<i32>], visited: &mut [Vec<bool>], row: i32, col: i32) -> i32 {
        let (num_rows, num_cols, mut fish_count) = (grid.len() as i32, grid[0].len() as i32, 0);
        let mut q = VecDeque::new();
        q.push_back((row, col));
        visited[row as usize][col as usize] = true;
        while let Some((row, col)) = q.pop_front() {
            fish_count += grid[row as usize][col as usize];
            for (mut new_row, mut new_col) in DIRECTIONS {
                (new_row, new_col) = (row + new_row, col + new_col);
                if 0 <= new_row
                    && new_row < num_rows
                    && 0 <= new_col
                    && new_col < num_cols
                    && grid[new_row as usize][new_col as usize] != 0
                    && !visited[new_row as usize][new_col as usize]
                {
                    q.push_back((new_row, new_col));
                    visited[new_row as usize][new_col as usize] = true;
                }
            }
        }
        fish_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::find_max_fish(arr![[0, 2, 1, 0], [4, 0, 0, 3], [1, 0, 0, 4], [0, 3, 2, 0]]),
            7
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::find_max_fish(arr![[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 1]]),
            1
        );
    }
}
