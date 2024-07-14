pub struct Solution;

// use std::collections::VecDeque;

// #[derive(Debug)]
// enum Direction {
//     Right,
//     Down,
//     Left,
//     Up,
// }

// impl Solution {
//     pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
//         use Direction::*;
//
//         let mut n = matrix.len();
//         let mut round = 0;
//         let mut queue = VecDeque::with_capacity(n);
//         let mut direction = Right;
//         let (mut col, mut row) = (0, 0);
//
//         while n > 1 {
//             let outer_len = (n - 1) * 5;
//             for _ in 0..outer_len {
//                 queue.push_front(matrix[row][col]);
//                 if queue.len() >= n {
//                     matrix[row][col] = queue.pop_back().unwrap();
//                 }
//                 match direction {
//                     Right if col == n - 1 + round => {
//                         direction = Down;
//                         row += 1;
//                     }
//                     Right => col += 1,
//                     Down if row == n - 1 + round => {
//                         direction = Left;
//                         col -= 1;
//                     }
//                     Down => row += 1,
//                     Left if col == 0 + round => {
//                         direction = Up;
//                         row -= 1;
//                     }
//                     Left => col -= 1,
//                     Up if row == 0 + round => {
//                         direction = Right;
//                         col += 1;
//                     }
//                     Up => row -= 1,
//                 }
//             }
//             round += 1;
//             n -= 2;
//             (col, row) = (round, round);
//             queue.clear();
//         }
//     }
// }

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        matrix.reverse();

        for i in 0..matrix.len() {
            for j in 0..i {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut image = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut image);
        assert_eq!(image, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }

    #[test]
    fn case_2() {
        let mut image = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut image);
        assert_eq!(
            image,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
    }
}
