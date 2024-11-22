#![allow(clippy::cast_sign_loss)]
pub struct Solution;

const UNGUARDED: i32 = 0;
const GUARDED: i32 = 1;
const GUARD: i32 = 2;
const WALL: i32 = 3;

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let mut grid = vec![vec![UNGUARDED; n as usize]; m as usize];
        for guard in &guards {
            grid[guard[0] as usize][guard[1] as usize] = GUARD;
        }
        for wall in walls {
            grid[wall[0] as usize][wall[1] as usize] = WALL;
        }
        for guard in guards {
            Self::mark_guarded(guard[0] as usize, guard[1] as usize, &mut grid);
        }
        let mut count = 0;
        for row in grid {
            for cell in row {
                if cell == UNGUARDED {
                    count += 1;
                }
            }
        }
        count
    }

    fn mark_guarded(row: usize, col: usize, grid: &mut [Vec<i32>]) {
        for r in (0..row).rev() {
            if grid[r][col] == WALL || grid[r][col] == GUARD {
                break;
            }
            grid[r][col] = GUARDED;
        }
        for r in grid.iter_mut().skip(row + 1) {
            if r[col] == WALL || r[col] == GUARD {
                break;
            }
            r[col] = GUARDED;
        }
        for c in (0..col).rev() {
            if grid[row][c] == WALL || grid[row][c] == GUARD {
                break;
            }
            grid[row][c] = GUARDED;
        }
        for c in col + 1..grid[row].len() {
            if grid[row][c] == WALL || grid[row][c] == GUARD {
                break;
            }
            grid[row][c] = GUARDED;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::count_unguarded(
                4,
                6,
                arr![[0, 0], [1, 1], [2, 3]],
                arr![[0, 1], [2, 2], [1, 4]]
            ),
            7
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::count_unguarded(3, 3, arr![[1, 1]], arr![[0, 1], [1, 0], [2, 1], [1, 2]]),
            4
        );
    }
}
