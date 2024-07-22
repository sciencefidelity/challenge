use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let i = intervals.partition_point(|v| v[1] < new_interval[0]);
        let j = intervals.partition_point(|v| v[0] <= new_interval[1]);

        let slice: Vec<_> = intervals.drain(i..j).flatten().collect();
        println!("{slice:?}");

        if let [first, .., last] = &slice[..] {
            new_interval[0] = min(new_interval[0], *first);
            new_interval[1] = max(new_interval[1], *last);
        }

        intervals.insert(i, new_interval);
        intervals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            vec![vec![1, 5], vec![6, 9]],
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5])
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            vec![vec![1, 2], vec![3, 10], vec![12, 16]],
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            )
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            vec![vec![1, 2], vec![4, 5], vec![7, 9]],
            Solution::insert(vec![vec![1, 2], vec![7, 9]], vec![4, 5])
        );
    }

    #[test]
    fn case_4() {
        assert_eq!(
            vec![vec![1, 5], vec![7, 9]],
            Solution::insert(vec![vec![1, 5], vec![7, 9]], vec![2, 5])
        );
    }
}
