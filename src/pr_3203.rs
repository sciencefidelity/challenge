#![allow(clippy::cast_sign_loss)]
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;
        let adj_list1 = Self::build_adj_list(n, &edges1);
        let adj_list2 = Self::build_adj_list(m, &edges2);
        let diameter1 = Self::find_diameter(n, &adj_list1);
        let diameter2 = Self::find_diameter(m, &adj_list2);
        ((diameter1 + 1) / 2 + (diameter2 + 1) / 2 + 1)
            .max(diameter1)
            .max(diameter2)
    }

    fn build_adj_list(size: usize, edges: &[Vec<i32>]) -> Vec<Vec<usize>> {
        let mut adj_list = vec![vec![]; size];
        for edge in edges {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            adj_list[u].push(v);
            adj_list[v].push(u);
        }
        adj_list
    }

    fn find_diameter(n: usize, adj_list: &[Vec<usize>]) -> i32 {
        let mut leaves_queue = VecDeque::new();
        let mut seen = vec![false; n];
        let (mut start, mut diameter) = (0, 0);
        for i in 0..2 {
            leaves_queue.push_back(start);
            seen[start] = true;
            let mut height = 0;
            while !leaves_queue.is_empty() {
                height += 1;
                for _ in 0..leaves_queue.len() {
                    let v = leaves_queue.pop_front().unwrap();
                    start = v;
                    for &u in &adj_list[v] {
                        if !seen[u] {
                            seen[u] = true;
                            leaves_queue.push_back(u);
                        }
                    }
                }
            }
            if i == 0 {
                seen.fill(false);
            } else {
                diameter = height - 1;
            }
        }
        diameter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::minimum_diameter_after_merge(arr![[0, 1], [0, 2], [0, 3]], arr![[0, 1]]),
            3
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::minimum_diameter_after_merge(
                arr![[0, 1], [0, 2], [0, 3], [2, 4], [2, 5], [3, 6], [2, 7]],
                arr![[0, 1], [0, 2], [0, 3], [2, 4], [2, 5], [3, 6], [2, 7]]
            ),
            5
        );
    }
}
