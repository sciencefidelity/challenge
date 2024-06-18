pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut complements = HashMap::new();
        let mut pairs = 0;
        for n in nums {
            match complements.get(&n) {
                Some(count) if *count > 0 => {
                    pairs += 1;
                    complements.insert(n, count - 1)
                }
                _ => {
                    let complement = k - n;
                    match complements.get(&complement) {
                        Some(count) => complements.insert(complement, count + 1),
                        None => complements.insert(complement, 1),
                    }
                }
            };
        }
        pairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::max_operations(vec![1, 2, 3, 4], 5), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::max_operations(vec![3, 1, 3, 4, 3], 6), 1);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::max_operations(vec![1, 3, 4, 2], 5), 2);
    }

    #[test]
    fn case_4() {
        assert_eq!(
            Solution::max_operations(vec![3, 1, 5, 1, 1, 1, 1, 1, 2, 2, 3, 2, 2], 1),
            0
        );
    }
}
