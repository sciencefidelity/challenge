#![allow(clippy::cast_sign_loss)]
pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn max_k_divisible_components(
        n: i32,
        edges: Vec<Vec<i32>>,
        values: Vec<i32>,
        k: i32,
    ) -> i32 {
        let mut adj_list = vec![Vec::new(); n as usize];
        for edge in edges {
            let (node_1, node_2) = (edge[0], edge[1]);
            adj_list[node_1 as usize].push(node_2);
            adj_list[node_2 as usize].push(node_1);
        }
        let mut component_count = 0;
        Self::dfs(0, -1, &adj_list, &values, k, &mut component_count);
        component_count
    }

    fn dfs(
        current_node: i32,
        parent_node: i32,
        adj_list: &[Vec<i32>],
        node_values: &[i32],
        k: i32,
        component_count: &mut i32,
    ) -> i32 {
        let mut sum = 0;
        for neighbor_node in &adj_list[current_node as usize] {
            if *neighbor_node != parent_node {
                sum += Self::dfs(
                    *neighbor_node,
                    current_node,
                    adj_list,
                    node_values,
                    k,
                    component_count,
                );
                sum %= k;
            }
        }
        sum += node_values[current_node as usize];
        sum %= k;
        if sum == 0 {
            *component_count += 1;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::max_k_divisible_components(
                5,
                arr![[0, 2], [1, 2], [1, 3], [2, 4]],
                vec![1, 8, 1, 4, 4],
                6
            ),
            2
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::max_k_divisible_components(
                7,
                arr![[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]],
                vec![3, 0, 6, 1, 5, 2, 1],
                3
            ),
            3
        );
    }
}
