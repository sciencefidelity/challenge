#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]
pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let (mut visit, mut in_stack) = (vec![false; n], vec![false; n]);
        for i in 0..n {
            Self::dfs(i, &graph, &mut visit, &mut in_stack);
        }
        let mut safe_nodes = Vec::new();
        for (i, item) in in_stack.iter().enumerate().take(n) {
            if !item {
                safe_nodes.push(i as i32);
            }
        }
        safe_nodes
    }

    fn dfs(
        node: usize,
        adj: &Vec<Vec<i32>>,
        visit: &mut Vec<bool>,
        in_stack: &mut Vec<bool>,
    ) -> bool {
        if in_stack[node] {
            return true;
        }
        if visit[node] {
            return false;
        }
        visit[node] = true;
        in_stack[node] = true;
        for neighbor in &adj[node] {
            if Self::dfs(*neighbor as usize, adj, visit, in_stack) {
                return true;
            }
        }
        in_stack[node] = false;
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::eventual_safe_nodes(arr![[1, 2], [2, 3], [5], [0], [5], [], []]),
            vec![2, 4, 5, 6]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::eventual_safe_nodes(arr![[1, 2, 3, 4], [1, 2], [3, 4], [0, 4], []]),
            vec![4]
        );
    }
}
