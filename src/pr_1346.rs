use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut set = HashSet::with_capacity(arr.len());
        for n in arr {
            if set.contains(&(n * 2)) || n % 2 == 0 && set.contains(&(n / 2)) {
                return true;
            }
            set.insert(n);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::check_if_exist(vec![10, 2, 5, 3]));
    }

    #[test]
    fn case_2() {
        assert!(!Solution::check_if_exist(vec![3, 1, 7, 11]));
    }
}
