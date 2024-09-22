#![allow(clippy::cast_sign_loss)]
pub struct Solution;

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = usize::try_from(n).unwrap();
        let mut adj = vec![vec![]; n];
        for connection in connections {
            adj[connection[0] as usize].push((connection[1], 1));
            adj[connection[1] as usize].push((connection[0], 0));
        }
        let mut count = 0;
        Self::dfs(0, -1, &adj, &mut count);
        count
    }

    fn dfs(node: i32, parent: i32, adj: &Vec<Vec<(i32, i32)>>, count: &mut i32) {
        for (neighbor, sign) in &adj[node as usize] {
            if *neighbor != parent {
                *count += sign;
                Self::dfs(*neighbor, node, adj, count);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::min_reorder(6, arr![[0, 1], [1, 3], [2, 3], [4, 0], [4, 5]]),
            3
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::min_reorder(5, arr![[1, 0], [1, 2], [3, 2], [3, 4]]),
            2
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::min_reorder(3, arr![[1, 0], [2, 0]]), 0);
    }
}
