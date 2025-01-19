#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut costs = vec![vec![i32::MAX; n]; m];
        let mut deque = VecDeque::new();

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let dir_map = [1, 2, 3, 4];

        costs[0][0] = 0;
        deque.push_back((0, 0, 0));

        while let Some((x, y, cost)) = deque.pop_front() {
            for (i, &(dx, dy)) in directions.iter().enumerate() {
                let new_x = (x as i32 + dx) as usize;
                let new_y = (y as i32 + dy) as usize;

                if new_x < m && new_y < n {
                    let new_cost = cost + i32::from(grid[x][y] != dir_map[i]);
                    if new_cost < costs[new_x][new_y] {
                        costs[new_x][new_y] = new_cost;
                        if grid[x][y] == dir_map[i] {
                            deque.push_front((new_x, new_y, new_cost));
                        } else {
                            deque.push_back((new_x, new_y, new_cost));
                        }
                    }
                }
            }
        }
        costs[m - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::min_cost(arr![[1, 1, 1, 1], [2, 2, 2, 2], [1, 1, 1, 1], [2, 2, 2, 2]]),
            3
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::min_cost(arr![[1, 1, 3], [3, 2, 2], [1, 1, 4]]), 0);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::min_cost(arr![[1, 2], [4, 3]]), 1);
    }
}
