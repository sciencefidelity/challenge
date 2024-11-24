pub struct Solution;

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let (mut sum, mut negatives, mut min) = (0, 0, i64::MAX);
        for row in matrix {
            for col in row {
                if col < 0 {
                    negatives += 1;
                }
                let col = i64::from(col.abs());
                min = min.min(col);
                sum += col;
            }
        }
        if negatives % 2 == 0 {
            sum
        } else {
            sum - min * 2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(Solution::max_matrix_sum(arr![[1, -1], [-1, 1]]), 4);
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::max_matrix_sum(arr![[1, 2, 3], [-1, -2, -3], [1, 2, 3]]),
            16
        );
    }
}
