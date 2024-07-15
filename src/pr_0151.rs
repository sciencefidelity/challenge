pub struct Solution;

// impl Solution {
//     pub fn reverse_words(s: String) -> String {
//         s.split_whitespace().rev().intersperse(" ").collect()
//     }
// }

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn reverse_words(s: String) -> String {
        let mut reversed_words = String::with_capacity(s.len());
        let mut iter = s.split_whitespace().peekable();
        while let Some(word) = iter.next_back() {
            reversed_words.push_str(word);
            if iter.peek().is_some() {
                reversed_words.push(' ');
            }
        }
        reversed_words
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".to_owned()),
            "blue is sky the".to_owned()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::reverse_words("  hello world  ".to_owned()),
            "world hello".to_owned()
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::reverse_words("a good   example".to_owned()),
            "example good a".to_owned()
        );
    }
}
