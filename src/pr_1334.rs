use std::cmp::{min, Reverse};

pub struct Solution;

impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = usize::try_from(n).unwrap();
        let mut distance_matrix = vec![vec![i32::MAX; n]; n];
        for (i, distance) in distance_matrix.iter_mut().enumerate() {
            distance[i] = 0;
        }
        for edge in edges {
            let start = usize::try_from(edge[0]).unwrap();
            let end = usize::try_from(edge[1]).unwrap();
            distance_matrix[start][end] = edge[2];
            distance_matrix[end][start] = edge[2];
        }
        Self::floyd(n, &mut distance_matrix);
        Self::get_city_with_fewest_reachable(n, &distance_matrix, distance_threshold)
    }

    fn floyd(n: usize, distance_matrix: &mut [Vec<i32>]) {
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    distance_matrix[i][j] = min(
                        distance_matrix[i][j],
                        distance_matrix[i][k].saturating_add(distance_matrix[k][j]),
                    );
                }
            }
        }
    }

    fn get_city_with_fewest_reachable(
        n: usize,
        shortest_path_matrix: &[Vec<i32>],
        distance_threshold: i32,
    ) -> i32 {
        (0..n)
            .min_by_key(|&i| {
                (
                    (0..n)
                        .filter(|&j| shortest_path_matrix[i][j] <= distance_threshold)
                        .count(),
                    Reverse(i),
                )
            })
            .unwrap()
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            3,
            Solution::find_the_city(
                4,
                vec![vec![0, 1, 3], vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 1]],
                4
            )
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            0,
            Solution::find_the_city(
                5,
                vec![
                    vec![0, 1, 2],
                    vec![0, 4, 8],
                    vec![1, 2, 3],
                    vec![1, 4, 2],
                    vec![2, 3, 1],
                    vec![3, 4, 1]
                ],
                2
            )
        );
    }
}
