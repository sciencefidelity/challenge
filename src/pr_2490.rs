pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn is_circular_sentence(sentence: String) -> bool {
        let sentence = sentence.as_bytes();
        for i in 0..sentence.len() {
            if sentence[i] == b' ' && sentence[i - 1] != sentence[i + 1] {
                return false;
            }
        }
        sentence[0] == sentence[sentence.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::is_circular_sentence(
            "leetcode exercises sound delightful".to_owned()
        ));
    }

    #[test]
    fn case_2() {
        assert!(Solution::is_circular_sentence("eetcode".to_owned()));
    }

    #[test]
    fn case_3() {
        assert!(!Solution::is_circular_sentence(
            "Leetcode is cool".to_owned()
        ));
    }
}
