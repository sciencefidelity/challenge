#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (rows, cols) = (is_water.len() as i32, is_water[0].len() as i32);
        let inf = rows * cols;
        let mut cell_heights = vec![vec![inf; is_water[0].len()]; is_water.len()];
        for row in 0..rows {
            for col in 0..cols {
                if is_water[row as usize][col as usize] != 0 {
                    cell_heights[row as usize][col as usize] = 0;
                }
            }
        }
        for row in 0..rows {
            for col in 0..cols {
                let mut min_neighbor_distance = inf;
                let mut neighbor_row = row - 1;
                let mut neighbor_col = col;
                if Self::is_valid_cell(neighbor_row, neighbor_col, rows, cols) {
                    min_neighbor_distance = min_neighbor_distance
                        .min(cell_heights[neighbor_row as usize][neighbor_col as usize]);
                }
                neighbor_row = row;
                neighbor_col = col - 1;
                if Self::is_valid_cell(neighbor_row, neighbor_col, rows, cols) {
                    min_neighbor_distance = min_neighbor_distance
                        .min(cell_heights[neighbor_row as usize][neighbor_col as usize]);
                }
                cell_heights[row as usize][col as usize] =
                    cell_heights[row as usize][col as usize].min(min_neighbor_distance + 1);
            }
        }
        for row in (0..rows).rev() {
            for col in (0..cols).rev() {
                let mut min_neighbor_distance = inf;
                let mut neighbor_row = row + 1;
                let mut neighbor_col = col;
                if Self::is_valid_cell(neighbor_row, neighbor_col, rows, cols) {
                    min_neighbor_distance = min_neighbor_distance
                        .min(cell_heights[neighbor_row as usize][neighbor_col as usize]);
                }
                neighbor_row = row;
                neighbor_col = col + 1;
                if Self::is_valid_cell(neighbor_row, neighbor_col, rows, cols) {
                    min_neighbor_distance = min_neighbor_distance
                        .min(cell_heights[neighbor_row as usize][neighbor_col as usize]);
                }
                cell_heights[row as usize][col as usize] =
                    cell_heights[row as usize][col as usize].min(min_neighbor_distance + 1);
            }
        }
        cell_heights
    }

    const fn is_valid_cell(row: i32, col: i32, rows: i32, cols: i32) -> bool {
        row >= 0 && col >= 0 && row < rows && col < cols
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::highest_peak(arr![[0, 1], [0, 0]]),
            arr![[1, 0], [2, 1]]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::highest_peak(arr![[0, 0, 1], [1, 0, 0], [0, 0, 0]]),
            arr![[1, 1, 0], [0, 1, 1], [1, 2, 2]]
        );
    }
}
