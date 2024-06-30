pub struct Solution;

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut alice_parents = (0..=n).collect::<Vec<usize>>();
        let mut bob_parents = alice_parents.clone();
        let mut alice_size = n;
        let mut bob_size = n;
        let total_edges = edges.len() as i32;
        let mut removed_edges = 0;

        let mut edges_type1 = Vec::new();
        let mut edges_type2 = Vec::new();
        let mut edges_type3 = Vec::new();

        for edge in edges {
            let edge_type = edge[0];
            let u = edge[1] as usize;
            let v = edge[2] as usize;

            match edge_type {
                1 => edges_type1.push((u, v)),
                2 => edges_type2.push((u, v)),
                _ => edges_type3.push((u, v)),
            }
        }

        for (u, v) in edges_type3.iter() {
            if Self::union(*u, *v, &mut alice_parents, &mut alice_size) {
                Self::union(*u, *v, &mut bob_parents, &mut bob_size);
                removed_edges += 1;
            }
        }

        for (u, v) in edges_type1.iter() {
            if Self::union(*u, *v, &mut alice_parents, &mut alice_size) {
                removed_edges += 1;
            }
        }

        for (u, v) in edges_type2.iter() {
            if Self::union(*u, *v, &mut bob_parents, &mut bob_size) {
                removed_edges += 1;
            }
        }

        if alice_size != 1 || bob_size != 1 {
            return -1;
        }

        total_edges - removed_edges
    }

    fn find(node: usize, parents: &mut Vec<usize>) -> usize {
        if parents[node] != node {
            parents[node] = Self::find(parents[node], parents);
        }
        parents[node]
    }

    fn union(node1: usize, node2: usize, parents: &mut Vec<usize>, size: &mut usize) -> bool {
        let root1 = Self::find(node1, parents);
        let root2 = Self::find(node2, parents);
        if root1 != root2 {
            *size -= 1;
            parents[root2] = root1;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::max_num_edges_to_remove(
                4,
                vec![
                    vec![3, 1, 2],
                    vec![3, 2, 3],
                    vec![1, 1, 3],
                    vec![1, 2, 4],
                    vec![1, 1, 2],
                    vec![2, 3, 4]
                ]
            ),
            2
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::max_num_edges_to_remove(
                4,
                vec![vec![3, 1, 2], vec![3, 2, 3], vec![1, 1, 4], vec![2, 1, 4]]
            ),
            0
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::max_num_edges_to_remove(4, vec![vec![3, 2, 3], vec![1, 1, 2], vec![2, 3, 4]]),
            -1
        );
    }
}
