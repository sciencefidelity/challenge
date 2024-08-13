use std::cmp::{max, min};
pub struct Solution;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut r_min_max, mut c_max_min) = (i32::MIN, i32::MAX);

        for i in 0..matrix[0].len() {
            let mut c_max = i32::MIN;
            for j in &matrix {
                c_max = max(c_max, j[i]);
            }
            c_max_min = min(c_max_min, c_max);
        }

        for i in matrix {
            let mut r_min = i32::MAX;
            for j in i {
                r_min = min(r_min, j);
            }
            r_min_max = max(r_min_max, r_min);
        }

        if r_min_max == c_max_min {
            return vec![r_min_max];
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]]),
            vec![15]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::lucky_numbers(vec![
                vec![1, 10, 4, 2],
                vec![9, 3, 8, 7],
                vec![15, 16, 17, 12]
            ]),
            vec![12]
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::lucky_numbers(vec![vec![7, 8], vec![1, 2]]),
            vec![7]
        );
    }

    #[test]
    fn case_4() {
        assert_eq!(
            Solution::lucky_numbers(vec![vec![3, 6], vec![7, 1], vec![5, 2], vec![4, 8]]),
            vec![]
        );
    }
}
