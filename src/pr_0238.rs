pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut products = vec![1_i32; nums.len()];
        let mut prefix = 1;
        let mut postfix = 1;

        for left in 0..nums.len() {
            let right = nums.len() - left - 1;

            products[left] *= prefix;
            products[right] *= postfix;

            prefix *= nums[left];
            postfix *= nums[right];
        }

        products
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}
