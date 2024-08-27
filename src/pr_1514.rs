use std::{cmp::Ordering, collections::BinaryHeap};

pub struct Solution;

#[derive(PartialEq)]
struct NonNan(f64);

impl PartialOrd for NonNan {
    #[allow(clippy::non_canonical_partial_ord_impl)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for NonNan {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for NonNan {}

impl Solution {
    #[allow(clippy::cast_sign_loss)]
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start_node: i32,
        end_node: i32,
    ) -> f64 {
        let (n, start, end) = (n as usize, start_node as usize, end_node as usize);
        let (mut graph, mut queue) = (vec![vec![]; n], BinaryHeap::with_capacity(n));
        for (edge, prob) in edges.into_iter().zip(succ_prob) {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            graph[u].push((v, prob));
            graph[v].push((u, prob));
        }
        let mut max_prob = vec![0.0; n];
        max_prob[start] = 1.0;
        queue.push((NonNan(1.0), start));
        while let Some((NonNan(cur_prob), cur_node)) = queue.pop() {
            if cur_node == end {
                return cur_prob;
            }
            for adj in &graph[cur_node] {
                if cur_prob * adj.1 > max_prob[adj.0] {
                    max_prob[adj.0] = cur_prob * adj.1;
                    queue.push((NonNan(max_prob[adj.0]), adj.0));
                }
            }
        }
        0.0
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::max_probability(3, arr![[0, 1], [1, 2], [0, 2]], vec![0.5, 0.5, 0.2], 0, 2),
            0.25
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::max_probability(3, arr![[0, 1], [1, 2], [0, 2]], vec![0.5, 0.5, 0.3], 0, 2),
            0.3
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::max_probability(3, arr![[0, 1]], vec![0.5], 0, 2),
            0.0
        );
    }
}
