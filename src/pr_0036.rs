pub struct Solution;

use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
enum Group {
    Horizontal,
    Vertical,
    Box,
}

// impl Solution {
//     pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
//         use Group::*;
//         let mut set = HashSet::new();
//
//         board
//             .into_iter()
//             .flatten()
//             .enumerate()
//             .map(|(n, k)| {
//                 let (i, j) = (n / 9, n % 9);
//                 k == '.'
//                     || set.insert((Horizontal, i, k))
//                         && set.insert((Vertical, j, k))
//                         && set.insert((Box, i / 3 * 3 + j / 3, k))
//             })
//             .all(|g| g)
//     }
// }

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut set = HashSet::new();

        for (r_idx, r_val) in board.into_iter().enumerate() {
            for (c_idx, c_val) in r_val.into_iter().enumerate() {
                if c_val == '.' {
                    continue;
                }
                let b_idx = 3 * (r_idx / 3) + c_idx / 3;
                if !set.insert((Group::Horizontal, r_idx, c_val))
                    || !set.insert((Group::Vertical, c_idx, c_val))
                    || !set.insert((Group::Box, b_idx, c_val))
                {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(Solution::is_valid_sudoku(board), true);
    }

    #[test]
    fn case_2() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(Solution::is_valid_sudoku(board), false);
    }
}
