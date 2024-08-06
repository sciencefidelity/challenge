pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn minimum_pushes(word: String) -> i32 {
        Self::helper(&word)
    }

    const fn helper(word: &str) -> i32 {
        let pushes = &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 12, 14, 16, 18, 20, 22, 24, 27, 30, 33, 36, 39, 42, 45,
            48, 52, 56,
        ];
        pushes[word.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(5, Solution::minimum_pushes("abcde".to_owned()));
    }

    #[test]
    fn case_2() {
        assert_eq!(12, Solution::minimum_pushes("xycdefghij".to_owned()));
    }
}
