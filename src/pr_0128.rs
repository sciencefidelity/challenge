pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let nums: HashSet<i32> = HashSet::from_iter(nums);
        let mut res = 0;
        for num in nums.iter() {
            if !nums.contains(&(num - 1)) {
                let mut count = num + 1;
                while nums.contains(&count) {
                    count += 1;
                }
                res = res.max(count - num);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }
}
