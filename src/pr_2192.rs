use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = usize::try_from(n).unwrap();
        let mut adjacency_list = vec![Vec::new(); n];
        let mut indegree = vec![0; n];
        for edge in edges {
            let from = usize::try_from(edge[0]).unwrap();
            let to = usize::try_from(edge[1]).unwrap();
            adjacency_list[from].push(to);
            indegree[to] += 1;
        }
        let mut ancestors_list = vec![Vec::new(); n];
        let mut nodes_with_zero_in_degree = indegree
            .iter()
            .enumerate()
            .filter(|(_, &degree)| degree == 0)
            .map(|(i, _)| i)
            .collect::<VecDeque<usize>>();
        while let Some(current_node) = nodes_with_zero_in_degree.pop_front() {
            ancestors_list[current_node].sort_unstable();
            ancestors_list[current_node].dedup();
            for &neighbor in &adjacency_list[current_node] {
                let ancestors = ancestors_list[current_node].clone();
                ancestors_list[neighbor].extend(ancestors);
                ancestors_list[neighbor].push(i32::try_from(current_node).unwrap());
                indegree[neighbor] -= 1;
                if indegree[neighbor] == 0 {
                    nodes_with_zero_in_degree.push_back(neighbor);
                }
            }
        }
        ancestors_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::get_ancestors(
                8,
                vec![
                    vec![0, 3],
                    vec![0, 4],
                    vec![1, 3],
                    vec![2, 4],
                    vec![2, 7],
                    vec![3, 5],
                    vec![3, 6],
                    vec![3, 7],
                    vec![4, 6]
                ]
            )[..8],
            vec![
                vec![],
                vec![],
                vec![],
                vec![0, 1],
                vec![0, 2],
                vec![0, 1, 3],
                vec![0, 1, 2, 3, 4],
                vec![0, 1, 2, 3]
            ]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::get_ancestors(
                8,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 3],
                    vec![0, 4],
                    vec![1, 2],
                    vec![1, 3],
                    vec![1, 4],
                    vec![2, 3],
                    vec![2, 4],
                    vec![3, 4]
                ]
            )[..5],
            vec![vec![], vec![0], vec![0, 1], vec![0, 1, 2], vec![0, 1, 2, 3],]
        );
    }
}
