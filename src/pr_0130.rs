pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (rows, cols) = (board.len(), board[0].len());
        let mut stack = Vec::with_capacity(rows * cols);
        for (row, col) in (0..cols)
            .map(|c| (0, c))
            .chain((0..cols).map(|c| (rows - 1, c)))
            .chain((1..rows - 1).map(|r| (r, 0)))
            .chain((1..rows - 1).map(|r| (r, cols - 1)))
        {
            if board[row][col] == 'O' {
                stack.push((row, col));
                while let Some((r, c)) = stack.pop() {
                    if r < rows && c < cols && board[r][c] == 'O' {
                        for d in [0, 1, 0, !0, 0].windows(2) {
                            stack.push((r.wrapping_add(d[0]), c.wrapping_add(d[1])));
                        }
                        board[r][c] = 'T';
                    }
                }
            }
        }
        for row in board.iter_mut() {
            for char in row.iter_mut() {
                match char {
                    char if *char == 'O' => *char = 'X',
                    char if *char == 'T' => *char = 'O',
                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        let expected = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut board);
        assert_eq!(expected, board);
    }

    #[test]
    fn case_2() {
        let mut board = vec![vec!['X']];
        let expected = vec![vec!['X']];
        Solution::solve(&mut board);
        assert_eq!(expected, board);
    }

    #[test]
    fn case_3() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        let expected = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut board);
        assert_eq!(expected, board);
    }

    #[test]
    fn case_4() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        let expected = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut board);
        assert_eq!(expected, board);
    }

    #[test]
    fn case_5() {
        let mut board = vec![
            vec!['X', 'X', 'X'],
            vec!['X', 'O', 'O'],
            vec!['X', 'O', 'O'],
            vec!['X', 'O', 'X'],
        ];
        let expected = vec![
            vec!['X', 'X', 'X'],
            vec!['X', 'O', 'O'],
            vec!['X', 'O', 'O'],
            vec!['X', 'O', 'X'],
        ];
        Solution::solve(&mut board);
        assert_eq!(expected, board);
    }
}
