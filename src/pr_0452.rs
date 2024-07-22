pub struct Solution;

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        if points.is_empty() {
            return 0;
        }
        points.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        let mut count = 1;
        let mut position = points[0][1];
        for point in points.iter().skip(1) {
            if position >= point[0] && position <= point[1] {
                continue;
            }
            count += 1;
            position = point[1];
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            2,
            Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]])
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            4,
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]])
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            2,
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]])
        );
    }
}
