pub struct Solution;

// impl Solution {
//     #[allow(clippy::needless_pass_by_value)]
//     pub fn decode_string(s: String) -> String {
//         let mut decoded = String::new();
//         let mut iter = s.chars().peekable();
//         let mut count: usize = 0;
//         while let Some(c) = iter.next() {
//             match c {
//                 c if c.is_ascii_digit() => {
//                     let mut num = String::from(c);
//                     while iter.peek().unwrap().is_ascii_digit() {
//                         num.push(iter.next().unwrap());
//                     }
//                     count = num.parse().unwrap();
//                 }
//                 '[' => {
//                     let mut inner_count = 0;
//                     let mut inner_str = String::new();
//                     for c in iter.by_ref() {
//                         if c == ']' && inner_count == 0 {
//                             break;
//                         }
//                         if c == '[' {
//                             inner_count += 1;
//                         }
//                         inner_str.push(c);
//                     }
//                     let inner_str = inner_str.repeat(count);
//                     decoded.push_str(&Self::decode_string(inner_str));
//                 }
//                 c if c.is_alphabetic() => {
//                     decoded.push(c);
//                 }
//                 _ => {}
//             }
//         }
//         decoded
//     }
// }

// impl Solution {
//     #[allow(clippy::needless_pass_by_value)]
//     pub fn decode_string(s: String) -> String {
//         let mut stack: Vec<String> = Vec::new();
//         let mut iter = s.chars().peekable();
//         while let Some(c) = iter.next() {
//             match c {
//                 ']' => {
//                     let mut str = String::new();
//                     while stack[stack.len() - 1] != "[" {
//                         str = stack.pop().unwrap() + &str;
//                     }
//                     stack.pop();
//                     let num = stack.pop().unwrap();
//                     stack.push(str.repeat(num.parse().unwrap()));
//                 }
//                 c if c.is_ascii_digit() => {
//                     let mut num = String::from(c);
//                     while iter.peek().unwrap().is_ascii_digit() {
//                         num.push(iter.next().unwrap());
//                     }
//                     stack.push(num);
//                 }
//                 c if c.is_ascii_alphabetic() => {
//                     let mut str = String::from(c);
//                     while iter.peek().is_some() && iter.peek().unwrap().is_ascii_alphabetic() {
//                         str.push(iter.next().unwrap());
//                     }
//                     stack.push(str);
//                 }
//                 '[' => stack.push('['.to_string()),
//                 _ => unreachable!(),
//             }
//         }
//         stack.join("")
//     }
// }

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn decode_string(s: String) -> String {
        let mut stack: Vec<String> = Vec::new();
        for c in s.chars() {
            if c == ']' {
                let mut substr = String::new();
                while stack[stack.len() - 1] != "[" {
                    substr = stack.pop().unwrap() + &substr;
                }
                stack.pop();
                let mut k = String::new();
                while !stack.is_empty() && stack[stack.len() - 1].parse::<i32>().is_ok() {
                    k = stack.pop().unwrap() + &k;
                }
                stack.push(substr.repeat(k.parse().unwrap()));
            } else {
                stack.push(c.to_string());
            }
        }
        stack.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            "aaabcbc".to_owned(),
            Solution::decode_string("3[a]2[bc]".to_owned())
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            "accaccacc".to_owned(),
            Solution::decode_string("3[a2[c]]".to_owned())
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            "abcabccdcdcdef".to_owned(),
            Solution::decode_string("2[abc]3[cd]ef".to_owned())
        );
    }
}
