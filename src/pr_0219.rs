pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = usize::try_from(k).unwrap();
        let mut map = HashMap::new();
        for (i, num) in nums.into_iter().enumerate() {
            if let Some(j) = map.insert(num, i) {
                if i - j <= k {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),);
    }

    #[test]
    fn case_2() {
        assert!(Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1),);
    }

    #[test]
    fn case_3() {
        assert!(!Solution::contains_nearby_duplicate(
            vec![1, 2, 3, 1, 2, 3],
            2
        ),);
    }
}
