use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub struct Solution;

impl Solution {
    #[allow(clippy::many_single_char_names)]
    pub fn modified_graph_edges(
        n: i32,
        edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
        target: i32,
    ) -> Vec<Vec<i32>> {
        let (n, source, destination) = (
            usize::try_from(n).unwrap(),
            usize::try_from(source).unwrap(),
            usize::try_from(destination).unwrap(),
        );
        let mut graph = vec![HashMap::new(); n];
        let mut s = vec![];
        for edge in &edges {
            if edge[2] == -1 {
                s.push((
                    usize::try_from(edge[0]).unwrap(),
                    usize::try_from(edge[1]).unwrap(),
                ));
            } else {
                graph[usize::try_from(edge[0]).unwrap()]
                    .insert(usize::try_from(edge[1]).unwrap(), edge[2]);
                graph[usize::try_from(edge[1]).unwrap()]
                    .insert(usize::try_from(edge[0]).unwrap(), edge[2]);
            }
        }
        let (mut dist, mut dist2) = (vec![0; n], vec![0; n]);
        Self::dfs(&graph, &mut dist, source);

        while dist[destination] > target && !s.is_empty() {
            Self::dfs(&graph, &mut dist2, destination);
            let (u, v) = s.pop().unwrap();
            let t = (dist[u] + dist2[v]).min(dist[v] + dist2[u]);
            let adjust = if t >= target { 1 } else { target - t };
            graph[u].insert(v, adjust);
            graph[v].insert(u, adjust);
            Self::dfs(&graph, &mut dist, source);
        }
        if dist[destination] != target {
            return Vec::new();
        }
        let mut result = Vec::new();
        for edge in edges {
            let (u, v) = (
                usize::try_from(edge[0]).unwrap(),
                usize::try_from(edge[1]).unwrap(),
            );
            if let Some(w) = graph[u].get(&v) {
                result.push(vec![edge[0], edge[1], *w]);
            } else {
                result.push(vec![edge[0], edge[1], 1_000_000_000]);
            }
        }
        result
    }

    fn dfs(graph: &[HashMap<usize, i32>], dist: &mut [i32], source: usize) {
        for u in dist.iter_mut().take(graph.len()) {
            *u = 1_000_000_000;
        }
        dist[source] = 0;
        let mut pq = BinaryHeap::new();
        pq.push(Reverse((0, source)));
        while let Some(Reverse((d, u))) = pq.pop() {
            if d > dist[u] {
                continue;
            }
            for p in &graph[u] {
                let (v, w) = (*p.0, *p.1);
                if dist[v] - dist[u] <= w {
                    continue;
                }
                dist[v] = w + dist[u];
                pq.push(Reverse((dist[v], v)));
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
        let edges = arr![[4, 1, -1], [2, 0, -1], [0, 3, -1], [4, 3, -1]];
        let expected = arr![[4, 1, 3], [2, 0, 1], [0, 3, 1], [4, 3, 1]];
        assert_eq!(Solution::modified_graph_edges(5, edges, 0, 1, 5), expected);
    }

    #[test]
    fn case_2() {
        let edges = arr![[0, 1, -1], [0, 2, 5]];
        let expected = Vec::<Vec<i32>>::new();
        assert_eq!(Solution::modified_graph_edges(3, edges, 0, 2, 6), expected);
    }

    #[test]
    fn case_3() {
        let edges = arr![[1, 0, 4], [1, 2, 3], [2, 3, 5], [0, 3, -1]];
        let expected = arr![[1, 0, 4], [1, 2, 3], [2, 3, 5], [0, 3, 1]];
        assert_eq!(Solution::modified_graph_edges(4, edges, 0, 2, 6), expected);
    }
}
