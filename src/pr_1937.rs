use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let cols = points[0].len();
        let (mut current_row, mut previous_row) = (vec![0; cols], vec![0; cols]);
        for row in points {
            let mut running_max = 0;
            for col in 0..cols {
                running_max = max(running_max - 1, previous_row[col]);
                current_row[col] = running_max;
            }
            running_max = 0;
            for col in (0..cols).rev() {
                running_max = max(running_max - 1, previous_row[col]);
                current_row[col] = max(current_row[col], running_max) + i64::from(row[col]);
            }
            previous_row.clone_from(&current_row);
        }
        let mut max_points: i64 = 0;
        for prev_col in previous_row {
            max_points = max(max_points, prev_col);
        }
        max_points
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            9,
            Solution::max_points(arr![[1, 2, 3], [1, 5, 1], [3, 1, 1]])
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(11, Solution::max_points(arr![[1, 5], [2, 3], [4, 2]]));
    }
}
