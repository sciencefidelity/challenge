#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
pub struct Solution;

impl Solution {
    pub fn rotate_the_box(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let (m, n) = (input.len(), input[0].len());
        let mut rotated = vec![vec!['.'; m]; n];
        for (i, row) in input.into_iter().enumerate() {
            let mut lowest_row_with_empty_cell = (n - 1) as i32;
            for (j, col) in row.into_iter().enumerate().rev() {
                if col == '#' {
                    rotated[lowest_row_with_empty_cell as usize][m - i - 1] = '#';
                    lowest_row_with_empty_cell -= 1;
                }
                if col == '*' {
                    rotated[j][m - i - 1] = '*';
                    lowest_row_with_empty_cell = j as i32 - 1;
                }
            }
        }
        rotated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input = vec![vec!['#', '.', '#']];
        let expected = vec![vec!['.'], vec!['#'], vec!['#']];
        assert_eq!(Solution::rotate_the_box(input), expected);
    }

    #[test]
    fn case_2() {
        let input = vec![vec!['#', '.', '*', '.'], vec!['#', '#', '*', '.']];
        let expected = vec![
            vec!['#', '.'],
            vec!['#', '#'],
            vec!['*', '*'],
            vec!['.', '.'],
        ];
        assert_eq!(Solution::rotate_the_box(input), expected);
    }

    #[test]
    fn case_3() {
        let input = vec![
            vec!['#', '#', '*', '.', '*', '.'],
            vec!['#', '#', '#', '*', '.', '.'],
            vec!['#', '#', '#', '.', '#', '.'],
        ];
        let expected = vec![
            vec!['.', '#', '#'],
            vec!['.', '#', '#'],
            vec!['#', '#', '*'],
            vec!['#', '*', '.'],
            vec!['#', '.', '*'],
            vec!['#', '.', '.'],
        ];
        assert_eq!(Solution::rotate_the_box(input), expected);
    }
}
