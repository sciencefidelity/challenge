use std::{collections::VecDeque, str::Chars};

pub struct Solution;

// impl Solution {
//     pub fn reverse_parentheses(s: String) -> String {
//         let mut open_parentheses_indicies = Vec::new();
//         let mut result = Vec::new();
//
//         for c in s.chars() {
//             match c {
//                 '(' => open_parentheses_indicies.push(result.len()),
//                 ')' => {
//                     let start = open_parentheses_indicies.pop().unwrap();
//                     let end = result.len() - 1;
//                     Self::reverse(&mut result, start, end);
//                 }
//                 c => result.push(c),
//             }
//         }
//
//         result.into_iter().collect()
//     }
//
//     fn reverse(chars: &mut Vec<char>, mut start: usize, mut end: usize) {
//         while start < end {
//             chars.swap(start, end);
//             start += 1;
//             end -= 1;
//         }
//     }
// }

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        Self::to_stack(&mut s.chars(), false).into_iter().collect()
    }

    fn to_stack(chars: &mut Chars, rev: bool) -> VecDeque<char> {
        let mut queue = VecDeque::new();
        loop {
            match chars.next() {
                None | Some(')') => break queue,
                Some('(') => {
                    let mut queue2 = Self::to_stack(chars, !rev);
                    if rev {
                        queue2.append(&mut queue);
                        queue = queue2;
                    } else {
                        queue.append(&mut queue2);
                    }
                }
                Some(c) => {
                    if rev {
                        queue.push_front(c);
                    } else {
                        queue.push_back(c);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::reverse_parentheses("(abcd)".to_owned()),
            "dcba".to_owned()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::reverse_parentheses("(u(love)i)".to_owned()),
            "iloveu".to_owned()
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::reverse_parentheses("(ed(et(oc))el)".to_owned()),
            "leetcode".to_owned()
        );
    }
}
