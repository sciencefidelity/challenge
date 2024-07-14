pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut rows = vec![false; matrix.len()];
        let mut cols = vec![false; matrix[0].len()];

        for row in 0..matrix.len() {
            for col in 0..matrix[0].len() {
                if matrix[row][col] == 0 {
                    rows[row] = true;
                    cols[col] = true;
                }
            }
        }
        for row in 0..matrix.len() {
            for col in 0..matrix[0].len() {
                if rows[row] || cols[col] {
                    matrix[row][col] = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let output = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, output);
    }

    #[test]
    fn case_2() {
        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        let output = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, output);
    }
}
