pub struct Solution;

use std::ops::AddAssign;

#[derive(Copy, Clone, PartialEq, Debug)]
struct Coordinates(isize, isize);

impl AddAssign for Coordinates {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (columns, rows) = (
            isize::try_from(matrix[0].len()).unwrap(),
            isize::try_from(matrix.len()).unwrap(),
        );

        let mut spiral_order = Vec::with_capacity(matrix.len() * matrix[0].len());
        let directions = [
            Coordinates(1, 0),
            Coordinates(0, 1),
            Coordinates(-1, 0),
            Coordinates(0, -1),
        ];

        let mut coordinates = Coordinates(0, 0);
        let mut round: isize = 0;

        let mut direction_iter = directions.into_iter().cycle();
        let mut direction = direction_iter.next().unwrap();

        for _ in 0..rows * columns {
            spiral_order.push(
                matrix[usize::try_from(coordinates.1).unwrap()]
                    [usize::try_from(coordinates.0).unwrap()],
            );

            if Self::is_border(columns, rows, round, &coordinates, &direction) {
                direction = direction_iter.next().unwrap();
                if direction == directions[3] {
                    round += 1;
                }
            }
            coordinates += direction;
        }
        spiral_order
    }

    const fn is_border(
        columns: isize,
        rows: isize,
        round: isize,
        coordinates: &Coordinates,
        direction: &Coordinates,
    ) -> bool {
        match direction {
            Coordinates(1, 0) if coordinates.0 == columns - 1 - round => true,
            Coordinates(0, 1) if coordinates.1 == rows - 1 - round => true,
            Coordinates(-1, 0) if coordinates.0 == round => true,
            Coordinates(0, -1) if coordinates.1 == round => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        let matrix = arr![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let spiral_flatten = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(Solution::spiral_order(matrix), spiral_flatten);
    }

    #[test]
    fn case_2() {
        let matrix = arr![[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
        let spiral_flatten = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];
        assert_eq!(Solution::spiral_order(matrix), spiral_flatten);
    }

    #[test]
    fn case_3() {
        let matrix = arr![
            [1, 2, 3, 4, 5],
            [6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15],
            [16, 17, 18, 19, 20],
            [21, 22, 23, 24, 25]
        ];
        let spiral_flatten = vec![
            1, 2, 3, 4, 5, 10, 15, 20, 25, 24, 23, 22, 21, 16, 11, 6, 7, 8, 9, 14, 19, 18, 17, 12,
            13,
        ];
        assert_eq!(Solution::spiral_order(matrix), spiral_flatten);
    }
}
