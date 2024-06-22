pub struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut ops = 0;
        let divisor = 3;
        for num in nums.iter() {
            let rem = num % divisor;
            match rem {
                1 | 2 => ops += 1,
                _ => {}
            }
        }
        ops
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::minimum_operations(vec![1, 2, 3, 4]), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::minimum_operations(vec![3, 6, 9]), 0);
    }
}
