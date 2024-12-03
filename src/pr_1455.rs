pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        for (word, i) in sentence.split_whitespace().zip(1_i32..) {
            if word.starts_with(&search_word) {
                return i;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::is_prefix_of_word("i love eating burger".to_owned(), "burg".to_owned()),
            4
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::is_prefix_of_word(
                "this problem is an easy problem".to_owned(),
                "pro".to_owned()
            ),
            2
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::is_prefix_of_word("i am tired".to_owned(), "you".to_owned()),
            -1
        );
    }
}
