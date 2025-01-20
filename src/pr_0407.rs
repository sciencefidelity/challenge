#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]
use std::cmp::Reverse;
use std::collections::binary_heap::BinaryHeap;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let dirs: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let (r, c) = (height_map.len(), height_map[0].len());
        let mut visited: Vec<Vec<bool>> = vec![vec![false; c]; r];
        let mut pq = BinaryHeap::new();
        for r in 0..height_map.len() {
            for c in 0..height_map[r].len() {
                if r > 0 && r < height_map.len() - 1 && c > 0 && c < height_map[r].len() - 1 {
                    continue;
                }
                visited[r][c] = true;
                pq.push(Reverse((height_map[r][c], r, c)));
            }
        }
        while !pq.is_empty() {
            match pq.pop() {
                Some(node) => {
                    let (cur_height, cur_row, cur_col) = ((node.0).0, (node.0).1, (node.0).2);
                    for dir in &dirs {
                        let dr = (cur_row as i32 + dir.0) as usize;
                        let dc = (cur_col as i32 + dir.1) as usize;
                        let is_not_border = dr > 0 && dr < r - 1 && dc > 0 && dc < c - 1;
                        if is_not_border && !visited[dr][dc] {
                            ans += std::cmp::max(0, cur_height - height_map[dr][dc]);
                            visited[dr][dc] = true;
                            pq.push(Reverse((
                                std::cmp::max(cur_height, height_map[dr][dc]),
                                dr,
                                dc,
                            )));
                        }
                    }
                }
                None => {
                    unreachable!();
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::trap_rain_water(arr![
                [1, 4, 3, 1, 3, 2],
                [3, 2, 1, 3, 2, 4],
                [2, 3, 3, 2, 3, 1]
            ]),
            4
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::trap_rain_water(arr![
                [3, 3, 3, 3, 3],
                [3, 2, 2, 2, 3],
                [3, 2, 1, 2, 3],
                [3, 2, 2, 2, 3],
                [3, 3, 3, 3, 3]
            ]),
            10
        );
    }
}
