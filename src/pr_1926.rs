#![allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let (m, n) = (maze.len(), maze[0].len());
        let mut queue = VecDeque::with_capacity(n * n);
        let start = (entrance[0] as usize, entrance[1] as usize);
        queue.push_back(start);
        maze[start.0][start.1] = '-';

        let is_exit = |point: (usize, usize)| -> bool {
            if point == start {
                return false;
            }
            if (point.0 == 0 || point.0 == m - 1) || (point.1 == 0 || point.1 == n - 1) {
                return true;
            }
            false
        };

        let directions = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

        let mut result = 0;
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let cur = queue.pop_front().unwrap();
                if is_exit(cur) {
                    return result;
                };
                for (dx, dy) in directions {
                    let new_x = (cur.0 as isize + dx) as usize;
                    let new_y = (cur.1 as isize + dy) as usize;
                    if new_x < m && new_y < n && maze[new_x][new_y] == '.' {
                        maze[new_x][new_y] = '-';
                        queue.push_back((new_x, new_y));
                    }
                }
            }
            result += 1;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::nearest_exit(
                vec![
                    vec!['+', '+', '.', '+'],
                    vec!['.', '.', '.', '+'],
                    vec!['+', '+', '+', '.']
                ],
                vec![1, 2]
            ),
            1
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::nearest_exit(
                vec![
                    vec!['+', '+', '+'],
                    vec!['.', '.', '.'],
                    vec!['+', '+', '+']
                ],
                vec![1, 0]
            ),
            2
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::nearest_exit(vec![vec!['.', '+'],], vec![0, 0]),
            -1
        );
    }
}
