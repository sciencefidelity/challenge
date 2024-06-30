pub struct Solution;

impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let mut degree: Vec<i64> = vec![0; n as usize];

        for edge in roads {
            degree[edge[0] as usize] += 1;
            degree[edge[1] as usize] += 1;
        }

        degree.sort_unstable();

        let mut value: i64 = 1;
        let mut total_importance: i64 = 0;
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
