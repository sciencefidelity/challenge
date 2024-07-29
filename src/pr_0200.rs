use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let (n_rows, n_cols) = (grid.len(), grid[0].len());
        let mut queue = VecDeque::with_capacity(n_rows * n_cols);
        let mut islands = 0;

        for i in 0..n_rows {
            for j in 0..n_cols {
                if grid[i][j] == '1' {
                    islands += 1;
                    queue.push_back((i, j));
                    while let Some((row, col)) = queue.pop_front() {
                        if row < n_rows && col < n_cols && grid[row][col] == '1' {
                            grid[row][col] = '0';
                            for w in [0, 1, 0, !0, 0].windows(2) {
                                queue.push_back((row.wrapping_add(w[0]), col.wrapping_add(w[1])));
                            }
                        }
                    }
                }
            }
        }
        islands
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(1, Solution::num_islands(grid));
    }

    #[test]
    fn case_2() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(3, Solution::num_islands(grid));
    }
}
