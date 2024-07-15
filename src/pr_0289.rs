use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let n = i32::try_from(board.len()).unwrap();
        let m = i32::try_from(if n > 0 { board[0].len() } else { 0 }).unwrap();

        for i in 0..n {
            for j in 0..m {
                let mut count = 0;
                for x in max(i - 1, 0)..min(i + 2, n) {
                    for y in max(j - 1, 0)..min(j + 2, m) {
                        count +=
                            board[usize::try_from(x).unwrap()][usize::try_from(y).unwrap()] & 1;
                    }
                }
                let (i, j) = (usize::try_from(i).unwrap(), usize::try_from(j).unwrap());
                if count == 3 || count - board[i][j] == 3 {
                    board[i][j] |= 2;
                }
            }
        }
        for i in 0..n {
            for j in 0..m {
                let (i, j) = (usize::try_from(i).unwrap(), usize::try_from(j).unwrap());
                board[i][j] >>= 1;
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
