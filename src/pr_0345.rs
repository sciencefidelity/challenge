pub struct Solution;

// impl Solution {
//     pub fn reverse_vowels(mut s: String) -> String {
//         let bytes = unsafe { s.as_bytes_mut() };
//
//         let mut iter = bytes.iter_mut();
//         while let (Some(left), Some(right)) = (iter.find(is_vowel), iter.rfind(is_vowel)) {
//             std::mem::swap(left, right);
//         }
//         s
//     }
// }

// fn is_vowel(c: &&mut u8) -> bool {
//     matches!(
//         c,
//         b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U'
//     )
// }

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut reversed_vowels = String::new();
        let mut stack: Vec<char> = Vec::new();
        let mut iter = s.chars();
        while let Some(c) = iter.next() {
            if is_vowel(c) {
                stack.push(c);
            }
        }
        let mut iter = s.chars();
        while let Some(c) = iter.next() {
            if is_vowel(c) {
                reversed_vowels.push(stack.pop().unwrap());
            } else {
                reversed_vowels.push(c);
            }
        }
        reversed_vowels
    }
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::reverse_vowels("hello".into()), "holle".to_owned());
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::reverse_vowels("leetcode".into()),
            "leotcede".to_owned()
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::reverse_vowels("aA".into()), "Aa".to_owned());
    }
}
