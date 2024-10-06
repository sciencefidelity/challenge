#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
use std::collections::VecDeque;

pub struct Solution;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut queue = VecDeque::with_capacity(rows * cols);
        let (mut time, mut fresh) = (0, 0);
        for (i, r) in grid.iter().enumerate() {
            for (j, c) in r.iter().enumerate() {
                if *c == 1 {
                    fresh += 1;
                }
                if *c == 2 {
                    queue.push_back((i as i32, j as i32));
                }
            }
        }
        while !queue.is_empty() && fresh > 0 {
            for _ in 0..queue.len() {
                if let Some((r, c)) = queue.pop_front() {
                    for (dr, dc) in DIRECTIONS {
                        let (row, col) = (dr + r, dc + c);
                        if row < 0
                            || row == rows as i32
                            || col < 0
                            || col == cols as i32
                            || grid[row as usize][col as usize] != 1
                        {
                            continue;
                        }
                        grid[row as usize][col as usize] = 2;
                        queue.push_back((row, col));
                        fresh -= 1;
                    }
                }
            }
            time += 1;
        }
        if fresh == 0 {
            time
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::oranges_rotting(arr![[2, 1, 1], [1, 1, 0], [0, 1, 1]]),
            4
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::oranges_rotting(arr![[2, 1, 1], [0, 1, 1], [1, 0, 1]]),
            -1
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::oranges_rotting(arr![[0, 2]]), 0);
    }
}
