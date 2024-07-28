use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let n = usize::try_from(n).unwrap();
        let mut adj = vec![vec![]; n + 1];
        for edge in edges {
            adj[usize::try_from(edge[0]).unwrap()].push(usize::try_from(edge[1]).unwrap());
            adj[usize::try_from(edge[1]).unwrap()].push(usize::try_from(edge[0]).unwrap());
        }

        let mut q = VecDeque::new();
        let (mut dist1, mut dist2) = (vec![-1; n + 1], vec![-1; n + 1]);
        q.push_back((1, 1));
        dist1[1] = 0;

        while let Some((node, freq)) = q.pop_front() {
            let mut time_taken = if freq == 1 { dist1[node] } else { dist2[node] };
            if (time_taken / change) % 2 == 0 {
                time_taken = time_taken + time;
            } else {
                time_taken = change * (time_taken / change + 1) + time;
            }
            for neighbor in &adj[node] {
                if dist1[*neighbor] == -1 {
                    dist1[*neighbor] = time_taken;
                    q.push_back((*neighbor, 1));
                } else if dist2[*neighbor] == -1 && dist1[*neighbor] != time_taken {
                    if *neighbor == n {
                        return time_taken;
                    }
                    dist2[*neighbor] = time_taken;
                    q.push_back((*neighbor, 2));
                }
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            13,
            Solution::second_minimum(
                5,
                vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![3, 4], vec![4, 5]],
                3,
                5
            )
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(11, Solution::second_minimum(2, vec![vec![1, 2]], 3, 2));
    }
}
