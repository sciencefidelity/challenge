use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let mut merged: Vec<Vec<i32>> = Vec::with_capacity(intervals.len());
        for interval in intervals {
            if merged.is_empty() || merged.last().unwrap()[1] < interval[0] {
                merged.push(interval);
            } else {
                merged.last_mut().unwrap()[1] = max(merged.last().unwrap()[1], interval[1]);
            }
        }
        merged
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let expected = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        let output = Solution::merge(input);
        assert_eq!(expected, output);
    }

    #[test]
    fn case_2() {
        let input = vec![vec![1, 4], vec![4, 5]];
        let expected = vec![vec![1, 5]];
        let output = Solution::merge(input);
        assert_eq!(expected, output);
    }
}
