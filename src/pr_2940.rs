#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]
pub struct Solution;

impl Solution {
    pub fn leftmost_building_queries(heights: Vec<i32>, mut queries: Vec<Vec<i32>>) -> Vec<i32> {
        for query in &mut queries {
            if query[0] > query[1] {
                query.swap(0, 1);
            }
        }
        let mut new_queries = vec![vec![]; heights.len()];
        let mut res = vec![-1; queries.len()];
        for (pos, query) in queries.into_iter().enumerate() {
            let (a, b) = (query[0] as usize, query[1] as usize);
            if a == b || heights[a] < heights[b] {
                res[pos] = b as i32;
            } else {
                new_queries[b].push((heights[a], pos));
            }
        }
        let mut stack = Vec::new();
        for (pos, height) in heights.into_iter().enumerate().rev() {
            for &(building_height, query_pos) in &new_queries[pos] {
                let pp = stack.partition_point(|(height, _)| building_height < *height);
                if pp > 0 {
                    res[query_pos] = stack[pp - 1].1 as i32;
                }
            }
            while !stack.is_empty() && height >= stack[stack.len() - 1].0 {
                stack.pop();
            }
            stack.push((height, pos));
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::leftmost_building_queries(
                vec![6, 4, 8, 5, 2, 7],
                arr![[0, 1], [0, 3], [2, 4], [3, 4], [2, 2]]
            ),
            vec![2, 5, -1, 5, 2]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::leftmost_building_queries(
                vec![5, 3, 8, 2, 6, 1, 4, 6],
                arr![[0, 7], [3, 5], [5, 2], [3, 0], [1, 6]]
            ),
            vec![7, 6, -1, 4, 6]
        );
    }
}
