pub struct Solution;

impl Solution {
    pub fn spiral_matrix_iii(
        rows: i32,
        cols: i32,
        mut r_start: i32,
        mut c_start: i32,
    ) -> Vec<Vec<i32>> {
        let dirs: &[(i32, i32); 4] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut traversed = Vec::with_capacity(usize::try_from(rows * cols).unwrap());
        let (mut step, mut direction) = (1, 0);
        while i32::try_from(traversed.len()).unwrap() < rows * cols {
            for _ in 0..2 {
                for _ in 0..step {
                    if r_start >= 0 && r_start < rows && c_start >= 0 && c_start < cols {
                        traversed.push(vec![r_start, c_start]);
                    }
                    r_start += dirs[direction].0;
                    c_start += dirs[direction].1;
                }
                direction = (direction + 1) % 4;
            }
            step += 1;
        }
        traversed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vectorize(v: &[&[i32; 2]]) -> Vec<Vec<i32>> {
        v.iter().map(|x| x.to_vec()).to_owned().collect()
    }

    #[test]
    fn case_1() {
        let expected = vectorize(&[&[0, 0], &[0, 1], &[0, 2], &[0, 3]]);
        assert_eq!(expected, Solution::spiral_matrix_iii(1, 4, 0, 0));
    }

    #[test]
    fn case_2() {
        let expected = vectorize(&[
            &[1, 4],
            &[1, 5],
            &[2, 5],
            &[2, 4],
            &[2, 3],
            &[1, 3],
            &[0, 3],
            &[0, 4],
            &[0, 5],
            &[3, 5],
            &[3, 4],
            &[3, 3],
            &[3, 2],
            &[2, 2],
            &[1, 2],
            &[0, 2],
            &[4, 5],
            &[4, 4],
            &[4, 3],
            &[4, 2],
            &[4, 1],
            &[3, 1],
            &[2, 1],
            &[1, 1],
            &[0, 1],
            &[4, 0],
            &[3, 0],
            &[2, 0],
            &[1, 0],
            &[0, 0],
        ]);
        assert_eq!(expected, Solution::spiral_matrix_iii(5, 6, 1, 4));
    }
}
