pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let mut x = x as i64;
        let a = x;
        while x > a / x {
            x = (x + a / x) / 2;
        }
        x as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::my_sqrt(4), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::my_sqrt(8), 2);
    }
}
