use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let n = board.len() as i32;
        let m = if n > 0 { board[0].len() } else { 0 } as i32;

        for i in 0..n {
            for j in 0..m {
                let mut count = 0;
                for x in max(i - 1, 0)..min(i + 2, n) {
                    for y in max(j - 1, 0)..min(j + 2, m) {
                        count += board[x as usize][y as usize] & 1;
                    }
                }
                if count == 3 || count - board[i as usize][j as usize] == 3 {
                    board[i as usize][j as usize] |= 2;
                }
            }
        }
        for i in 0..n {
            for j in 0..m {
                board[i as usize][j as usize] >>= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut input = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        let output = vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]];
        Solution::game_of_life(&mut input);
        assert_eq!(input, output);
    }

    #[test]
    fn case_2() {
        let mut input = vec![vec![1, 1], vec![1, 0]];
        let output = vec![vec![1, 1], vec![1, 1]];
        Solution::game_of_life(&mut input);
        assert_eq!(input, output);
    }
}
