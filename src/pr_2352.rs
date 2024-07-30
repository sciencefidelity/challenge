use std::collections::HashMap;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let (mut pairs, mut map) = (0, HashMap::new());
        for row in &grid {
            map.entry(row).and_modify(|count| *count += 1).or_insert(1);
        }
        for j in 0..cols {
            let mut col = Vec::new();
            #[allow(clippy::needless_range_loop)]
            for i in 0..rows {
                col.push(grid[i][j]);
            }
            pairs += map.get(&col).unwrap_or(&0);
        }
        pairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let grid = vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]];
        assert_eq!(1, Solution::equal_pairs(grid));
    }

    #[test]
    fn case_2() {
        let grid = vec![
            vec![3, 1, 2, 2],
            vec![1, 4, 4, 5],
            vec![2, 4, 2, 2],
            vec![2, 4, 2, 2],
        ];
        assert_eq!(3, Solution::equal_pairs(grid));
    }

    #[test]
    fn case_3() {
        let grid = vec![
            vec![3, 1, 2, 2],
            vec![1, 4, 4, 4],
            vec![2, 4, 2, 2],
            vec![2, 5, 2, 2],
        ];
        assert_eq!(3, Solution::equal_pairs(grid));
    }
}
