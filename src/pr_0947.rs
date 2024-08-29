pub struct Solution;

// impl Solution {
//     #[allow(clippy::needless_pass_by_value)]
//     pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
//         let n = stones.len();
//         let mut adjacency_list = vec![vec![]; n];
//         for i in 0..n {
//             for j in (i + 1)..n {
//                 if stones[i][0] == stones[j][0] || stones[i][1] == stones[j][1] {
//                     adjacency_list[i].push(j);
//                     adjacency_list[j].push(i);
//                 }
//             }
//         }
//         let (mut num_connected_components, mut visited) = (0, vec![false; n]);
//         for i in 0..n {
//             if !visited[i] {
//                 Self::dfs(&adjacency_list, &mut visited, i);
//                 num_connected_components += 1;
//             }
//         }
//         i32::try_from(n - num_connected_components).unwrap()
//     }
//
//     fn dfs(adjacency_list: &Vec<Vec<usize>>, visited: &mut Vec<bool>, stone: usize) {
//         visited[stone] = true;
//         for neighbor in &adjacency_list[stone] {
//             if !visited[*neighbor] {
//                 Self::dfs(adjacency_list, visited, *neighbor);
//             }
//         }
//     }
// }

use std::collections::HashSet;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let (n, mut uf) = (stones.len(), UnionFind::new(20002));
        for stone in stones {
            uf.union(stone[0], stone[1] + 10001);
        }
        i32::try_from(n - uf.component_count).unwrap()
    }
}

struct UnionFind {
    parent: Vec<i32>,
    component_count: usize,
    unique_nodes: HashSet<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: vec![-1; n],
            component_count: 0,
            unique_nodes: HashSet::new(),
        }
    }

    fn find(&mut self, node: i32) -> i32 {
        if !self.unique_nodes.contains(&node) {
            self.component_count += 1;
            self.unique_nodes.insert(node);
        }
        let unode = usize::try_from(node).unwrap();
        if self.parent[unode] == -1 {
            return node;
        }
        self.parent[unode] = self.find(self.parent[unode]);
        self.parent[unode]
    }

    fn union(&mut self, node1: i32, node2: i32) {
        let root1 = self.find(node1);
        let root2 = self.find(node2);
        if root1 == root2 {
            return;
        }
        self.parent[usize::try_from(root1).unwrap()] = root2;
        self.component_count -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        let stones = arr![[0, 0], [0, 1], [1, 0], [1, 2], [2, 1], [2, 2]];
        assert_eq!(Solution::remove_stones(stones), 5);
    }

    #[test]
    fn case_2() {
        let stones = arr![[0, 0], [0, 2], [1, 1], [2, 0], [2, 2]];
        assert_eq!(Solution::remove_stones(stones), 3);
    }

    #[test]
    fn case_3() {
        let stones = arr![[0, 0]];
        assert_eq!(Solution::remove_stones(stones), 0);
    }
}
