#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]
use std::collections::VecDeque;

pub struct Solution;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut min_obstacles = vec![vec![i32::MAX; n]; m];
        min_obstacles[0][0] = 0;
        let mut deque = VecDeque::from([(0, 0, 0)]);
        while !deque.is_empty() {
            let current = deque.pop_front().unwrap();
            let (obstacles, row, col) = (current.0, current.1, current.2);
            for dir in DIRECTIONS {
                let (new_row, new_col) = (row + dir.0, col + dir.1);
                if Self::is_valid(&grid, new_row, new_col)
                    && min_obstacles[new_row as usize][new_col as usize] == i32::MAX
                {
                    if grid[new_row as usize][new_col as usize] == 1 {
                        min_obstacles[new_row as usize][new_col as usize] = obstacles + 1;
                        deque.push_back((obstacles + 1, new_row, new_col));
                    } else {
                        min_obstacles[new_row as usize][new_col as usize] = obstacles;
                        deque.push_front((obstacles, new_row, new_col));
                    }
                }
            }
        }
        min_obstacles[m - 1][n - 1]
    }

    fn is_valid(grid: &[Vec<i32>], row: i32, col: i32) -> bool {
        row >= 0 && col >= 0 && row < grid.len() as i32 && col < grid[0].len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::minimum_obstacles(arr![[0, 1, 1], [1, 1, 0], [1, 1, 0]]),
            2
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::minimum_obstacles(arr![[0, 1, 0, 0, 0], [0, 1, 0, 1, 0], [0, 0, 0, 1, 0]]),
            0
        );
    }
}
