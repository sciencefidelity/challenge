use std::cmp::max;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut max_cur, mut max_so_far) = (0, 0);
        for i in 1..prices.len() {
            max_cur += prices[i] - prices[i - 1];
            max_cur = max(0, max_cur);
            max_so_far = max(max_cur, max_so_far);
        }
        max_so_far
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(5, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    }

    #[test]
    fn case_2() {
        assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
    }
}
