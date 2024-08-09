pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        if m < 3 || n < 3 {
            return 0;
        }
        let mut count = 0;
        for row in 0..m - 2 {
            for col in 0..n - 2 {
                if Self::is_magic_square(&grid, row, col) {
                    count += 1;
                }
            }
        }
        count
    }

    fn is_magic_square(grid: &[Vec<i32>], row: usize, col: usize) -> bool {
        let sequence = "2943816729438167";
        let sequence_reversed = "7618349276183492";
        let mut border = String::new();
        let border_indicies = [0, 1, 2, 5, 8, 7, 6, 3];
        for i in border_indicies {
            let num = grid[row + i / 3][col + (i % 3)];
            border += &num.to_string();
        }
        grid[row][col] % 2 == 0
            && grid[row + 1][col + 1] == 5
            && (sequence.contains(&border) || sequence_reversed.contains(&border))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let grid = vec![vec![4, 3, 8, 4], vec![9, 5, 1, 9], vec![2, 7, 6, 2]];
        assert_eq!(1, Solution::num_magic_squares_inside(grid));
    }

    #[test]
    fn case_2() {
        let grid = vec![vec![8]];
        assert_eq!(0, Solution::num_magic_squares_inside(grid));
    }
}
