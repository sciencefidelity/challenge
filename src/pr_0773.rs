#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
use std::collections::{HashSet, VecDeque};

pub struct Solution;

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let directions: Vec<Vec<usize>> = vec![
            vec![1, 3],
            vec![0, 2, 4],
            vec![1, 5],
            vec![0, 4],
            vec![1, 3, 5],
            vec![2, 4],
        ];
        let target = vec![1, 2, 3, 4, 5, 0];
        let mut start_state = Vec::with_capacity(6);
        for row in board {
            for col in row {
                start_state.push(col as u8);
            }
        }
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(start_state.clone());
        visited.insert(start_state.clone());
        let mut moves = 0;
        while !queue.is_empty() {
            let mut size = queue.len();
            while size > 0 {
                let current_state = queue.front().unwrap().clone();
                queue.pop_front();
                if *current_state == target {
                    return moves;
                }
                let zero_pos = current_state.iter().position(|&r| r == 0).unwrap();
                for new_pos in &directions[zero_pos] {
                    let mut next_state = current_state.clone();
                    (next_state[zero_pos], next_state[*new_pos]) =
                        (next_state[*new_pos], next_state[zero_pos]);
                    if visited.contains(&next_state) {
                        continue;
                    }
                    visited.insert(next_state.clone());
                    queue.push_back(next_state.clone());
                }
                size -= 1;
            }
            moves += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(Solution::sliding_puzzle(arr![[1, 2, 3], [4, 0, 5]]), 1);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::sliding_puzzle(arr![[1, 2, 3], [5, 4, 0]]), -1);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::sliding_puzzle(arr![[4, 1, 2], [5, 0, 3]]), 5);
    }
}
