pub struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let n = usize::try_from(n).unwrap();
        Self::helper(n)[n]
    }

    const fn helper(n: usize) -> [i32; 38] {
        let mut memo = [0; 38];
        let mut i = 1;
        while i <= n {
            if i < 3 {
                memo[i] = 1;
            } else {
                memo[i] = memo[i - 3] + memo[i - 2] + memo[i - 1];
            }
            i += 1;
        }
        memo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(4, Solution::tribonacci(4));
    }

    #[test]
    fn case_2() {
        assert_eq!(1_389_537, Solution::tribonacci(25));
    }

    #[test]
    fn case_3() {
        assert_eq!(615_693_474, Solution::tribonacci(35));
    }

    #[test]
    fn case_4() {
        assert_eq!(2_082_876_103, Solution::tribonacci(37));
    }
}
