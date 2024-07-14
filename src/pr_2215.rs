pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let nums1 = nums1.into_iter().collect::<HashSet<_>>();
        let nums2 = nums2.into_iter().collect::<HashSet<_>>();
        vec![
            nums1.difference(&nums2).copied().collect(),
            nums2.difference(&nums1).copied().collect(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let solution = Solution::find_difference(vec![1, 2, 3], vec![2, 4, 6]);
        assert_eq!(solution.len(), 2);
        assert_eq!(solution[0].len(), 2);
        assert_eq!(solution[1].len(), 2);
        assert!(solution[0].contains(&1));
        assert!(solution[0].contains(&3));
        assert!(solution[1].contains(&4));
        assert!(solution[1].contains(&6));
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::find_difference(vec![1, 2, 3, 3], vec![1, 1, 2, 2]),
            vec![vec![3], vec![]]
        );
    }
}
