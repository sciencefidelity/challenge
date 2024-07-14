pub struct Solution;

use std::collections::HashMap;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut map = HashMap::new();
        s.bytes().enumerate().for_each(|(i, c)| {
            map.insert(c, i32::try_from(i).unwrap());
        });
        t.bytes().enumerate().fold(0, |acc, (i, c)| {
            if let Some(diff) = map.get(&c) {
                return acc + (*diff - i32::try_from(i).unwrap()).abs();
            }
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::find_permutation_difference("abc".to_owned(), "bac".to_owned()),
            2
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::find_permutation_difference("abcde".to_owned(), "edbac".to_owned()),
            12
        );
    }
}
