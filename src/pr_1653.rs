use std::cmp::min;

pub struct Solution;

// impl Solution {
//     #[allow(clippy::needless_pass_by_value)]
//     pub fn minimum_deletions(s: String) -> i32 {
//         let n = s.len();
//         let mut stack = Vec::with_capacity(n);
//         let mut delete_count = 0;
//
//         for b in s.bytes() {
//             if !stack.is_empty() && stack[stack.len() - 1] == b'b' && b == b'a' {
//                 stack.pop();
//                 delete_count += 1;
//             } else {
//                 stack.push(b);
//             }
//         }
//         delete_count
//     }
// }

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn minimum_deletions(s: String) -> i32 {
        let mut min_deletions = 0;
        let mut b_count = 0;

        for b in s.bytes() {
            if b == b'b' {
                b_count += 1;
            } else {
                min_deletions = min(min_deletions + 1, b_count);
            }
        }
        min_deletions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(2, Solution::minimum_deletions("aababbab".to_owned()));
    }

    #[test]
    fn case_2() {
        assert_eq!(2, Solution::minimum_deletions("bbaaaaabb".to_owned()));
    }
}
