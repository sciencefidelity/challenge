pub struct Solution;

impl Solution {
    pub fn count_sub_islands(mut grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let m = i32::try_from(grid1.len()).unwrap();
        let n = i32::try_from(grid1[0].len()).unwrap();
        for (i, row) in grid2.into_iter().enumerate() {
            for (j, cell) in row.into_iter().enumerate() {
                if cell == 1 {
                    if grid1[i][j] == 1 {
                        grid1[i][j] = 2;
                    } else {
                        grid1[i][j] = 3;
                    }
                }
            }
        }
        let mut num_islands = 0;
        for i in 0..m {
            for j in 0..n {
                if grid1[usize::try_from(i).unwrap()][usize::try_from(j).unwrap()] == 2 {
                    let mut covered = true;
                    Self::is_covered(&mut grid1, i, j, m, n, &mut covered);
                    if covered {
                        num_islands += 1;
                    }
                }
            }
        }
        num_islands
    }

    fn is_covered(grid: &mut Vec<Vec<i32>>, i: i32, j: i32, m: i32, n: i32, covered: &mut bool) {
        if i < 0
            || i >= m
            || j < 0
            || j >= n
            || grid[usize::try_from(i).unwrap()][usize::try_from(j).unwrap()] <= 1
        {
            return;
        }
        if grid[usize::try_from(i).unwrap()][usize::try_from(j).unwrap()] == 3 {
            *covered = false;
        }
        grid[usize::try_from(i).unwrap()][usize::try_from(j).unwrap()] = 0;
        Self::is_covered(grid, i + 1, j, m, n, covered);
        Self::is_covered(grid, i - 1, j, m, n, covered);
        Self::is_covered(grid, i, j + 1, m, n, covered);
        Self::is_covered(grid, i, j - 1, m, n, covered);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        let grid1 = arr![
            [1, 1, 1, 0, 0],
            [0, 1, 1, 1, 1],
            [0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 1, 0, 1, 1]
        ];
        let grid2 = arr![
            [1, 1, 1, 0, 0],
            [0, 0, 1, 1, 1],
            [0, 1, 0, 0, 0],
            [1, 0, 1, 1, 0],
            [0, 1, 0, 1, 0]
        ];
        assert_eq!(Solution::count_sub_islands(grid1, grid2), 3);
    }

    #[test]
    fn case_2() {
        let grid1 = arr![
            [1, 0, 1, 0, 1],
            [1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1],
            [1, 0, 1, 0, 1]
        ];
        let grid2 = arr![
            [0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [1, 0, 0, 0, 1]
        ];
        assert_eq!(Solution::count_sub_islands(grid1, grid2), 2);
    }

    #[test]
    fn case_3() {
        let grid1 = arr![
            [1, 1, 1, 1, 0, 0],
            [1, 1, 0, 1, 0, 0],
            [1, 0, 0, 1, 1, 1],
            [1, 1, 1, 0, 0, 1],
            [1, 1, 1, 1, 1, 0],
            [1, 0, 1, 0, 1, 0],
            [0, 1, 1, 1, 0, 1],
            [1, 0, 0, 0, 1, 1],
            [1, 0, 0, 0, 1, 0],
            [1, 1, 1, 1, 1, 0]
        ];
        let grid2 = arr![
            [1, 1, 1, 1, 0, 1],
            [0, 0, 1, 0, 1, 0],
            [1, 1, 1, 1, 1, 1],
            [0, 1, 1, 1, 1, 1],
            [1, 1, 1, 0, 1, 0],
            [0, 1, 1, 1, 1, 1],
            [1, 1, 0, 1, 1, 1],
            [1, 0, 0, 1, 0, 1],
            [1, 1, 1, 1, 1, 1],
            [1, 0, 0, 1, 0, 0]
        ];
        assert_eq!(Solution::count_sub_islands(grid1, grid2), 0);
    }
}
