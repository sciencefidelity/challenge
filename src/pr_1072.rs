use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut pattern_frequency = HashMap::with_capacity(matrix[0].len());
        for current_row in matrix {
            let mut pattern_builder = Vec::with_capacity(current_row.len());
            for col in 0..current_row.len() {
                if current_row[0] == current_row[col] {
                    pattern_builder.push(b'T');
                } else {
                    pattern_builder.push(b'F');
                }
            }
            pattern_frequency
                .entry(pattern_builder)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        *pattern_frequency.values().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::max_equal_rows_after_flips(arr![[0, 1], [1, 1]]),
            1
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::max_equal_rows_after_flips(arr![[0, 1], [1, 0]]),
            2
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::max_equal_rows_after_flips(arr![[0, 0, 0], [0, 0, 1], [1, 1, 0]]),
            2
        );
    }
}
