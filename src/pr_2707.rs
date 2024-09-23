// use std::collections::{HashMap, HashSet};
//
// pub struct Solution;
//
// impl Solution {
//     #[allow(clippy::needless_pass_by_value)]
//     pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
//         let mut dp = Dp::new(&s, &dictionary);
//         dp.run(0).try_into().unwrap()
//     }
// }
//
// struct Dp<'a> {
//     s: &'a str,
//     n: usize,
//     set: HashSet<&'a str>,
//     memo: HashMap<usize, usize>,
// }
//
// impl<'a> Dp<'a> {
//     pub fn new(s: &'a str, dictionary: &'a [String]) -> Self {
//         Self {
//             s,
//             n: s.len(),
//             set: dictionary.iter().map(String::as_str).collect(),
//             memo: HashMap::new(),
//         }
//     }
//
//     pub fn run(&mut self, start: usize) -> usize {
//         if start == self.n {
//             return 0;
//         }
//         if self.memo.contains_key(&start) {
//             return *self.memo.get(&start).unwrap();
//         }
//         let mut result = self.run(start + 1) + 1;
//         for end in start..self.n {
//             let curr = &self.s[start..=end];
//             if self.set.contains(curr) {
//                 result = result.min(self.run(end + 1));
//             }
//         }
//         self.memo.insert(start, result);
//         result
//     }
// }

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let n = s.len();
        let set: HashSet<&str> = dictionary.iter().map(String::as_str).collect();
        let mut dp = vec![0; n + 1];

        for start in (0..n).rev() {
            dp[start] = dp[start + 1] + 1;
            for end in start..n {
                let curr = &s[start..=end];
                if set.contains(curr) {
                    dp[start] = dp[start].min(dp[end + 1]);
                }
            }
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::min_extra_char("leetscode".to_owned(), arr!["leet", "code", "leetcode"]),
            1
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::min_extra_char("sayhelloworld".to_owned(), arr!["hello", "world"]),
            3
        );
    }
}
