pub struct Solution;

use std::collections::{BTreeSet, HashMap};

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut occurrences: HashMap<i32, u16> = HashMap::new();
        let mut unique: BTreeSet<u16> = BTreeSet::new();
        for i in 0..arr.len() {
            *occurrences.entry(arr[i]).or_insert(0) += 1;
        }
        for num_occurrences in occurrences.values() {
            if !unique.insert(*num_occurrences) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]), true);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::unique_occurrences(vec![1, 2]), false);
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]),
            true
        );
    }
}
