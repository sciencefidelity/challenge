pub struct Solution;

// use std::cmp::{max, min};

// impl Solution {
//     pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
//         let pattern_x = ['a', 'b'];
//         let pattern_y = ['b', 'a'];
//         let (high_priority_pair, low_priority_pair) = if x > y {
//             (&pattern_x, &pattern_y)
//         } else {
//             (&pattern_y, &pattern_x)
//         };
//         let mut total_score = 0;
//
//         let string_after_first_pass = Self::remove_substring(&s, high_priority_pair);
//         let removed_pairs_count = (s.len() - string_after_first_pass.len()) as i32 / 2;
//         total_score += removed_pairs_count * max(x, y);
//
//         let string_after_second_pass =
//             Self::remove_substring(&string_after_first_pass, low_priority_pair);
//         let removed_pairs_count =
//             (string_after_first_pass.len() - string_after_second_pass.len()) as i32 / 2;
//         total_score += removed_pairs_count * min(x, y);
//
//         total_score
//     }
//
//     fn remove_substring(input: &str, target_pair: &[char; 2]) -> String {
//         let mut char_stack: Vec<char> = Vec::with_capacity(input.len());
//         for c in input.chars() {
//             if c == target_pair[1]
//                 && !char_stack.is_empty()
//                 && *char_stack.last().unwrap() == target_pair[0]
//             {
//                 char_stack.pop();
//             } else {
//                 char_stack.push(c);
//             }
//         }
//         String::from_iter(char_stack)
//     }
// }

use std::cmp::min;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn maximum_gain(s: String, mut x: i32, mut y: i32) -> i32 {
        let mut direction = 1;
        if x < y {
            (x, y, direction) = (y, x, -1);
        }

        let (mut a_count, mut b_count, mut total_points) = (0, 0, 0);
        let mut s = s.chars();
        while let Some(c) = if direction == 1 {
            s.next()
        } else {
            s.next_back()
        } {
            match c {
                'a' => a_count += 1,
                'b' if a_count > 0 => {
                    a_count -= 1;
                    total_points += x;
                }
                'b' => b_count += 1,
                _ => {
                    total_points += min(b_count, a_count) * y;
                    (a_count, b_count) = (0, 0);
                }
            }
        }

        total_points += min(b_count, a_count) * y;
        total_points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::maximum_gain("cdbcbbaaabab".into(), 4, 5), 19);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::maximum_gain("aabbaaxybbaabb".into(), 5, 4), 20);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::maximum_gain("bbaabbxyaabbaa".into(), 4, 5), 20);
    }
}
