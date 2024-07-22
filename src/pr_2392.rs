use std::collections::VecDeque;

pub struct Solution;

type Vec2D = Vec<Vec<i32>>;

impl Solution {
    pub fn build_matrix(k: i32, row_conditions: Vec2D, col_conditions: Vec2D) -> Vec2D {
        let k = usize::try_from(k).unwrap();
        let order_rows = Self::topo_sort(row_conditions, k);
        let order_cols = Self::topo_sort(col_conditions, k);

        if order_rows.is_empty() || order_cols.is_empty() {
            return vec![];
        }
        let mut matrix: Vec2D = vec![vec![0; k]; k];

        for (i, row) in matrix.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                if order_rows[i] == order_cols[j] {
                    *col = i32::try_from(order_rows[i]).unwrap();
                }
            }
        }

        matrix
    }

    fn topo_sort(edges: Vec2D, mut n: usize) -> Vec<usize> {
        let mut adj = vec![vec![]; n + 1];
        let (mut deg, mut order) = (vec![0; n + 1], vec![]);
        for x in edges {
            adj[usize::try_from(x[0]).unwrap()].push(usize::try_from(x[1]).unwrap());
            deg[usize::try_from(x[1]).unwrap()] += 1;
        }
        let mut q = VecDeque::new();
        for (i, val) in deg.iter().enumerate().take(n + 1).skip(1) {
            if *val == 0 {
                q.push_back(i);
            }
        }
        while !q.is_empty() {
            let f = q.pop_front().unwrap();
            order.push(f);
            n -= 1;
            for v in &adj[f] {
                deg[*v] -= 1;
                if deg[*v] == 0 {
                    q.push_back(*v);
                }
            }
        }
        if n != 0 {
            return vec![];
        }
        order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let expected = vec![vec![3, 0, 0], vec![0, 0, 1], vec![0, 2, 0]];
        let output = Solution::build_matrix(
            3,
            vec![vec![1, 2], vec![3, 2]],
            vec![vec![2, 1], vec![3, 2]],
        );
        assert_eq!(output.len(), 3);
        for row in expected {
            assert!(output.contains(&row));
        }
    }

    #[test]
    fn case_2() {
        let expected: Vec2D = vec![];
        let output = Solution::build_matrix(
            3,
            vec![vec![1, 2], vec![2, 3], vec![3, 1], vec![2, 1]],
            vec![vec![2, 1]],
        );
        assert_eq!(expected, output);
    }
}
