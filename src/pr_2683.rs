pub struct Solution;

impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        let mut sum = 0;
        for num in derived {
            sum += num;
        }
        sum % 2 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::does_valid_array_exist(vec![1, 1, 0]));
    }

    #[test]
    fn case_2() {
        assert!(Solution::does_valid_array_exist(vec![1, 1]));
    }

    #[test]
    fn case_3() {
        assert!(!Solution::does_valid_array_exist(vec![1, 0]));
    }
}
