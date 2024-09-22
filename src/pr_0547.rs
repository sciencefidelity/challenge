pub struct Solution;

// impl Solution {
//     #[allow(clippy::needless_pass_by_value)]
//     pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
//         let (mut num_components, mut visit) = (0, vec![false; is_connected.len()]);
//         for i in 0..is_connected.len() {
//             if !visit[i] {
//                 num_components += 1;
//                 Self::dfs(i, &is_connected, &mut visit);
//             }
//         }
//         num_components
//     }
//
//     fn dfs(node: usize, is_connected: &[Vec<i32>], visit: &mut [bool]) {
//         visit[node] = true;
//         for i in 0..is_connected.len() {
//             if is_connected[node][i] != 0 && !visit[i] {
//                 Self::dfs(i, is_connected, visit);
//             }
//         }
//     }
// }

// use std::collections::VecDeque;
//
// impl Solution {
//     #[allow(clippy::needless_pass_by_value)]
//     pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
//         let (mut num_components, mut visit) = (0, vec![false; is_connected.len()]);
//         for i in 0..is_connected.len() {
//             if !visit[i] {
//                 num_components += 1;
//                 Self::bfs(i, &is_connected, &mut visit);
//             }
//         }
//         num_components
//     }
//
//     fn bfs(node: usize, is_connected: &Vec<Vec<i32>>, visit: &mut Vec<bool>) {
//         let mut queue = VecDeque::from([node]);
//         visit[node] = true;
//         while let Some(node) = queue.pop_front() {
//             for i in 0..is_connected.len() {
//                 if is_connected[node][i] != 0 && !visit[i] {
//                     queue.push_back(i);
//                     visit[i] = true;
//                 }
//             }
//         }
//     }
// }

use std::cmp::Ordering;

pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<i32>,
    count: usize,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        let (mut parent, rank, count) = (vec![0; size], vec![0; size], size);
        for (i, p) in parent.iter_mut().enumerate() {
            *p = i;
        }
        Self {
            parent,
            rank,
            count,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union_set(&mut self, x: usize, y: usize) {
        let (xset, yset) = (self.find(x), self.find(y));
        match self.rank[xset].cmp(&self.rank[yset]) {
            Ordering::Less => self.parent[xset] = yset,
            Ordering::Greater => self.parent[yset] = xset,
            Ordering::Equal => {
                self.parent[yset] = xset;
                self.rank[xset] += 1;
            }
        }
    }
}

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let (mut dsu, mut num_components) = (UnionFind::new(n), n);
        for (i, row) in is_connected.iter().enumerate() {
            for (j, col) in row.iter().enumerate().skip(i + 1) {
                if *col != 0 && dsu.find(i) != dsu.find(j) {
                    num_components -= 1;
                    dsu.union_set(i, j);
                }
            }
        }
        i32::try_from(num_components).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::find_circle_num(arr![[1, 1, 0], [1, 1, 0], [0, 0, 1]]),
            2
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::find_circle_num(arr![[1, 0, 0], [0, 1, 0], [0, 0, 1]]),
            3
        );
    }
}
