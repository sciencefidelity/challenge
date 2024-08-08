pub struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let ns = usize::try_from(n).unwrap();
        let mut matrix = vec![vec![0; ns]; ns];
        let dirs: &[(i32, i32); 4] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];
        let (mut current_dir_idx, mut row, mut col) = (0, 0, 0);
        for i in 1..=n.pow(2) {
            matrix[usize::try_from(row).unwrap()][usize::try_from(col).unwrap()] = i;
            let (mut dx, mut dy) = dirs[current_dir_idx];
            if Self::is_out_of_bounds(n, row + dx, col + dy)
                || matrix[usize::try_from(row + dx).unwrap()][usize::try_from(col + dy).unwrap()]
                    > 0
            {
                current_dir_idx = (current_dir_idx + 1) % 4;
            }
            (dx, dy) = dirs[current_dir_idx];
            (row, col) = (row + dx, col + dy);
        }
        matrix
    }

    const fn is_out_of_bounds(n: i32, row: i32, col: i32) -> bool {
        row < 0 || row >= n || col < 0 || col >= n
    }

    const fn idx_convert_1d_2d(n: i32, idx: i32) -> (i32, i32) {
        (idx / n, idx % n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let expected = vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        assert_eq!(expected, Solution::generate_matrix(3));
    }

    #[test]
    fn case_2() {
        let expected = vec![vec![1]];
        assert_eq!(expected, Solution::generate_matrix(1));
    }
}
