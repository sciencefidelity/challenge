pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut rows = vec![false; matrix.len()];
        let mut cols = vec![false; matrix[0].len()];

        for (i_row, v_row) in matrix.iter().enumerate() {
            for (i_col, v_col) in v_row.iter().enumerate() {
                if *v_col == 0 {
                    rows[i_row] = true;
                    cols[i_col] = true;
                }
            }
        }
        for (i_row, v_row) in matrix.iter_mut().enumerate() {
            for (i_col, v_col) in v_row.iter_mut().enumerate() {
                if rows[i_row] || cols[i_col] {
                    *v_col = 0;
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
