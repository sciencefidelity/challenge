pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let (mut t, mut b, mut r) = (
            grid[0].iter().fold(0i64, |acc, x| acc + i64::from(*x)),
            0,
            i64::MAX,
        );
        for (i, j) in grid[0].iter().zip(grid[1].iter()) {
            t -= i64::from(*i);
            r = r.min(t.max(b));
            b += i64::from(*j);
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(Solution::grid_game(arr![[2, 5, 4], [1, 5, 1]]), 4);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::grid_game(arr![[3, 3, 1], [8, 5, 2]]), 4);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::grid_game(arr![[1, 3, 1, 15], [1, 3, 3, 1]]), 7);
    }
}
