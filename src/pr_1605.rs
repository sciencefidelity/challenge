use std::cmp::min;

pub struct Solution;

impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut matrix: Vec<Vec<i32>> = vec![vec![0; col_sum.len()]; row_sum.len()];
        for (i, r_val) in row_sum.iter_mut().enumerate() {
            for (j, c_val) in col_sum.iter_mut().enumerate() {
                matrix[i][j] = min(*r_val, *c_val);
                *r_val -= matrix[i][j];
                *c_val -= matrix[i][j];
            }
        }
        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sum_rows_columns(matrix: Vec<Vec<i32>>) -> (Vec<i32>, Vec<i32>) {
        let (mut row_sums, mut col_sums): (Vec<i32>, Vec<i32>) = (vec![], vec![]);
        for row in &matrix {
            row_sums.push(row.iter().sum::<i32>());
        }
        for i in 0..matrix[0].len() {
            let mut sum = 0;
            for j in 0..matrix.len() {
                sum += matrix[j][i];
            }
            col_sums.push(sum);
        }
        (row_sums, col_sums)
    }

    #[test]
    fn case_1() {
        let input = (vec![3, 8], vec![4, 7]);
        let output = Solution::restore_matrix(input.0, input.1);
        assert_eq!((vec![3, 8], vec![4, 7]), sum_rows_columns(output));
    }

    #[test]
    fn case_2() {
        let input = (vec![5, 7, 10], vec![8, 6, 8]);
        let output = Solution::restore_matrix(input.0, input.1);
        assert_eq!((vec![5, 7, 10], vec![8, 6, 8]), sum_rows_columns(output));
    }

    #[test]
    fn case_3() {
        let input = (vec![8, 6, 8], vec![5, 7, 10]);
        let output = Solution::restore_matrix(input.0, input.1);
        assert_eq!((vec![8, 6, 8], vec![5, 7, 10]), sum_rows_columns(output));
    }
}
