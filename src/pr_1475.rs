pub struct Solution;

impl Solution {
    pub fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
        let n = prices.len();
        let mut stack = Vec::with_capacity(n);
        for i in 0..n {
            while !stack.is_empty() && prices[stack[stack.len() - 1]] >= prices[i] {
                prices[stack[stack.len() - 1]] -= prices[i];
                stack.pop();
            }
            stack.push(i);
        }
        prices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::final_prices(vec![8, 4, 6, 2, 3]),
            vec![4, 2, 4, 2, 3]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::final_prices(vec![1, 2, 3, 4, 5]),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::final_prices(vec![10, 1, 1, 6]), vec![9, 0, 1, 6]);
    }
}
