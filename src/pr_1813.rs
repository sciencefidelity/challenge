pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let mut words1 = sentence1.split_ascii_whitespace().peekable();
        let mut words2 = sentence2.split_ascii_whitespace().peekable();
        while let (Some(w1), Some(w2)) = (words1.peek(), words2.peek()) {
            if w1 != w2 {
                break;
            }
            words1.next();
            words2.next();
        }
        while let (Some(w1), Some(w2)) = (words1.next_back(), words2.next_back()) {
            if w1 != w2 {
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
        assert!(Solution::are_sentences_similar(
            "My name is Haley".to_owned(),
            "My Haley".to_owned()
        ));
    }

    #[test]
    fn case_2() {
        assert!(!Solution::are_sentences_similar(
            "of".to_owned(),
            "A lot of words".to_owned()
        ));
    }

    #[test]
    fn case_3() {
        assert!(Solution::are_sentences_similar(
            "Eating right now".to_owned(),
            "Eating".to_owned()
        ));
    }
}
