#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][1] > 1 && grid[1][0] > 1 {
            return -1;
        }

        let (m, n) = (grid.len(), grid[0].len());
        let mut q = BinaryHeap::from([Reverse((grid[0][0], 0, 0))]);
        let mut visited = HashSet::from([(0, 0)]);
        let mut res = -1;

        while let Some(Reverse((time, x, y))) = q.pop() {
            if x == m - 1 && y == n - 1 {
                res = time;
                break;
            }

            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                if nx < m && ny < n && !visited.contains(&(nx, ny)) {
                    visited.insert((nx, ny));
                    let move_time = i32::from((grid[nx][ny] - time) % 2 == 0);
                    q.push(Reverse(((time + 1).max(grid[nx][ny] + move_time), nx, ny)));
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::minimum_time(arr![[0, 1, 3, 2], [5, 1, 2, 5], [4, 3, 8, 6]]),
            7
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::minimum_time(arr![[0, 2, 4], [3, 2, 1], [1, 0, 4]]),
            -1
        );
    }
}
