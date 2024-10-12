#![allow(clippy::cast_sign_loss)]
pub struct Solution;

impl Solution {
    pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
        let (mut range_start, mut range_end) = (i32::MAX, i32::MIN);
        for interval in &intervals {
            (range_start, range_end) = (range_start.min(interval[0]), range_end.max(interval[1]));
        }
        let mut point_to_count = vec![0; range_end as usize + 2];
        for interval in intervals {
            point_to_count[interval[0] as usize] += 1;
            point_to_count[interval[1] as usize + 1] -= 1;
        }
        let mut concurrent_intervals = 0;
        let mut max_concurrent_intervals = 0;
        for i in range_start..=range_end {
            concurrent_intervals += point_to_count[i as usize];
            max_concurrent_intervals = max_concurrent_intervals.max(concurrent_intervals);
        }
        max_concurrent_intervals
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::min_groups(arr![[5, 10], [6, 8], [1, 5], [2, 3], [1, 10]]),
            3
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::min_groups(arr![[1, 3], [5, 6], [8, 10], [11, 13]]),
            1
        );
    }
}
