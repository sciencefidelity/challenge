pub struct Solution;

impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let mut degree = vec![0; usize::try_from(n).unwrap()];

        for edge in roads {
            degree[usize::try_from(edge[0]).unwrap()] += 1;
            degree[usize::try_from(edge[1]).unwrap()] += 1;
        }
        degree.sort_unstable();

        let mut value = 1;
        let mut total_importance = 0;
        for d in degree {
            total_importance += value * d;
            value += 1;
        }
        total_importance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::maximum_importance(
                5,
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![2, 3],
                    vec![0, 2],
                    vec![1, 3],
                    vec![2, 4]
                ]
            ),
            43
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::maximum_importance(5, vec![vec![0, 3], vec![2, 4], vec![1, 3]]),
            20
        );
    }
}
