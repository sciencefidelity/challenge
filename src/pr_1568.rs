use std::cmp::min;

pub struct Solution;

const DIRECTIONS: &[(i32, i32); 4] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];

struct ArticulationPointInfo {
    has_articulation_point: bool,
    time: i32,
}

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn min_days(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut ap_info = ArticulationPointInfo {
            has_articulation_point: false,
            time: 0,
        };
        let (mut land_cells, mut island_count) = (0, 0);
        let mut discovery_time = vec![vec![-1; cols]; rows];
        let mut lowest_reachable = vec![vec![-1; cols]; rows];
        let mut parent_cell = vec![vec![-1; cols]; rows];
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == 1 {
                    land_cells += 1;
                    if discovery_time[i][j] == -1 {
                        let (i, j) = (i32::try_from(i).unwrap(), i32::try_from(j).unwrap());
                        Self::find_articulation_points(
                            &grid,
                            i,
                            j,
                            &mut discovery_time,
                            &mut lowest_reachable,
                            &mut parent_cell,
                            &mut ap_info,
                        );
                        island_count += 1;
                    }
                }
            }
        }
        if island_count == 0 || island_count >= 2 {
            return 0;
        }
        if land_cells == 1 {
            return 1;
        }
        if ap_info.has_articulation_point {
            return 1;
        }
        2
    }

    fn find_articulation_points(
        grid: &[Vec<i32>],
        row: i32,
        col: i32,
        discovery_time: &mut Vec<Vec<i32>>,
        lowest_reachable: &mut Vec<Vec<i32>>,
        parent_cell: &mut Vec<Vec<i32>>,
        ap_info: &mut ArticulationPointInfo,
    ) {
        let cols = i32::try_from(grid[0].len()).unwrap();
        discovery_time[usize::try_from(row).unwrap()][usize::try_from(col).unwrap()] = ap_info.time;
        ap_info.time += 1;
        lowest_reachable[usize::try_from(row).unwrap()][usize::try_from(col).unwrap()] =
            discovery_time[usize::try_from(row).unwrap()][usize::try_from(col).unwrap()];
        let mut children = 0;
        for direction in DIRECTIONS {
            let new_row = row + direction.0;
            let new_col = col + direction.1;
            if Self::is_valid_land_cell(grid, new_row, new_col) {
                if discovery_time[usize::try_from(new_row).unwrap()]
                    [usize::try_from(new_col).unwrap()]
                    == -1
                {
                    children += 1;
                    parent_cell[usize::try_from(new_row).unwrap()]
                        [usize::try_from(new_col).unwrap()] = row * cols + col;
                    Self::find_articulation_points(
                        grid,
                        new_row,
                        new_col,
                        discovery_time,
                        lowest_reachable,
                        parent_cell,
                        ap_info,
                    );
                    lowest_reachable[usize::try_from(row).unwrap()]
                        [usize::try_from(col).unwrap()] = min(
                        lowest_reachable[usize::try_from(row).unwrap()]
                            [usize::try_from(col).unwrap()],
                        lowest_reachable[usize::try_from(new_row).unwrap()]
                            [usize::try_from(new_col).unwrap()],
                    );
                    if lowest_reachable[usize::try_from(new_row).unwrap()]
                        [usize::try_from(new_col).unwrap()]
                        >= discovery_time[usize::try_from(row).unwrap()]
                            [usize::try_from(col).unwrap()]
                        && parent_cell[usize::try_from(row).unwrap()][usize::try_from(col).unwrap()]
                            != -1
                    {
                        ap_info.has_articulation_point = true;
                    }
                } else if new_row * cols + new_col
                    != parent_cell[usize::try_from(row).unwrap()][usize::try_from(col).unwrap()]
                {
                    lowest_reachable[usize::try_from(row).unwrap()]
                        [usize::try_from(col).unwrap()] = min(
                        lowest_reachable[usize::try_from(row).unwrap()]
                            [usize::try_from(col).unwrap()],
                        discovery_time[usize::try_from(new_row).unwrap()]
                            [usize::try_from(new_col).unwrap()],
                    );
                }
            }
        }
        if parent_cell[usize::try_from(row).unwrap()][usize::try_from(col).unwrap()] == -1
            && children > 1
        {
            ap_info.has_articulation_point = true;
        }
    }

    fn is_valid_land_cell(grid: &[Vec<i32>], row: i32, col: i32) -> bool {
        let rows = i32::try_from(grid.len()).unwrap();
        let cols = i32::try_from(grid[0].len()).unwrap();
        row >= 0
            && col >= 0
            && row < rows
            && col < cols
            && grid[usize::try_from(row).unwrap()][usize::try_from(col).unwrap()] == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            2,
            Solution::min_days(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]])
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(2, Solution::min_days(vec![vec![1, 1]]));
    }
}
