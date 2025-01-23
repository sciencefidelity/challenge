#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut communicatable_servers_count = 0;
        let mut row_counts = vec![0; grid[0].len()];
        let mut last_server_in_col = vec![-1_i32; grid.len()];
        for (i, row) in grid.iter().enumerate() {
            let mut server_count_in_row = 0;
            for (j, col) in row.iter().enumerate() {
                if *col == 1 {
                    server_count_in_row += 1;
                    row_counts[j] += 1;
                    last_server_in_col[i] = j as i32;
                }
            }
            if server_count_in_row != 1 {
                communicatable_servers_count += server_count_in_row;
                last_server_in_col[i] = -1;
            }
        }
        for i in 0..grid.len() {
            if last_server_in_col[i] != -1 && row_counts[last_server_in_col[i] as usize] > 1 {
                communicatable_servers_count += 1;
            }
        }
        communicatable_servers_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(Solution::count_servers(arr![[1, 0], [0, 1]]), 0);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::count_servers(arr![[1, 0], [1, 1]]), 3);
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::count_servers(arr![[1, 1, 0, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 0, 1]]),
            4
        );
    }
}
