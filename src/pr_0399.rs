use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut adj = HashMap::new();
        for (i, eq) in equations.iter().enumerate() {
            let (a, b) = (&eq[0], &eq[1]);
            adj.entry(a)
                .and_modify(|v: &mut Vec<(&String, f64)>| v.push((b, values[i])))
                .or_insert_with(|| vec![(b, values[i])]);
            adj.entry(b)
                .and_modify(|v: &mut Vec<(&String, f64)>| v.push((a, 1.0 / values[i])))
                .or_insert_with(|| vec![(a, 1.0 / values[i])]);
        }
        let mut output = Vec::new();
        for q in queries {
            output.push(Self::bfs(&adj, &q[0], &q[1]));
        }
        output
    }

    fn bfs(adj: &HashMap<&String, Vec<(&String, f64)>>, src: &String, target: &String) -> f64 {
        if !adj.contains_key(src) || !adj.contains_key(target) {
            return -1.0;
        }
        let (mut q, mut visit) = (VecDeque::new(), HashSet::new());
        q.push_back((src, 1.0));
        visit.insert(src);
        while let Some((n, w)) = q.pop_front() {
            if n == target {
                return w;
            }
            for (neighbor, weight) in &adj[n] {
                if !visit.contains(neighbor) {
                    q.push_back((neighbor, w * weight));
                    visit.insert(neighbor);
                }
            }
        }
        -1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let equations = vec![
            vec!["a".to_owned(), "b".to_owned()],
            vec!["b".to_owned(), "c".to_owned()],
        ];
        let values = vec![2.0, 3.0];
        let queries = vec![
            vec!["a".to_owned(), "c".to_owned()],
            vec!["b".to_owned(), "a".to_owned()],
            vec!["a".to_owned(), "e".to_owned()],
            vec!["a".to_owned(), "a".to_owned()],
            vec!["x".to_owned(), "x".to_owned()],
        ];
        let expected = vec![6.00000, 0.50000, -1.00000, 1.00000, -1.00000];
        assert_eq!(
            expected,
            Solution::calc_equation(equations, values, queries)
        );
    }

    #[test]
    fn case_2() {
        let equations = vec![
            vec!["a".to_owned(), "b".to_owned()],
            vec!["b".to_owned(), "c".to_owned()],
            vec!["bc".to_owned(), "cd".to_owned()],
        ];
        let values = vec![1.5, 2.5, 5.0];
        let queries = vec![
            vec!["a".to_owned(), "c".to_owned()],
            vec!["c".to_owned(), "b".to_owned()],
            vec!["bc".to_owned(), "cd".to_owned()],
            vec!["cd".to_owned(), "bc".to_owned()],
        ];
        let expected = vec![3.75000, 0.40000, 5.00000, 0.20000];
        assert_eq!(
            expected,
            Solution::calc_equation(equations, values, queries)
        );
    }

    #[test]
    fn case_3() {
        let equations = vec![vec!["a".to_owned(), "b".to_owned()]];
        let values = vec![0.5];
        let queries = vec![
            vec!["a".to_owned(), "b".to_owned()],
            vec!["b".to_owned(), "a".to_owned()],
            vec!["a".to_owned(), "c".to_owned()],
            vec!["x".to_owned(), "y".to_owned()],
        ];
        let expected = vec![0.50000, 2.00000, -1.00000, -1.00000];
        assert_eq!(
            expected,
            Solution::calc_equation(equations, values, queries)
        );
    }
}
