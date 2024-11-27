#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]
pub struct Solution;

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut answer = Vec::with_capacity(queries.len());
        let mut adj_list = vec![Vec::new(); n];
        for (i, item) in adj_list.iter_mut().enumerate() {
            item.push(i + 1);
        }
        for road in queries {
            let u = road[0] as usize;
            let v = road[1] as usize;
            adj_list[u].push(v);
            answer.push(Self::find_min_distance(&adj_list, n) as i32);
        }

        answer
    }

    fn find_min_distance(adj_list: &[Vec<usize>], n: usize) -> usize {
        let mut dp = vec![0; n];
        for current_node in (0..n - 1).rev() {
            let mut min_distance = n;
            for neighbor in &adj_list[current_node] {
                min_distance = min_distance.min(dp[*neighbor] + 1);
            }
            dp[current_node] = min_distance;
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::shortest_distance_after_queries(5, arr![[2, 4], [0, 2], [0, 4]]),
            vec![3, 2, 1]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::shortest_distance_after_queries(4, arr![[0, 3], [0, 2]]),
            vec![1, 1]
        );
    }
}
