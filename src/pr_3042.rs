pub struct Solution;

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut count = 0;
        for i in 0..words.len() {
            for j in i + 1..words.len() {
                count += Self::is_prefix_and_suffix(&words[i], &words[j]) as i32;
            }
        }
        count
    }

    fn is_prefix_and_suffix(a: &str, b: &str) -> bool {
        b.starts_with(a) && b.ends_with(a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::count_prefix_suffix_pairs(arr!["a", "aba", "ababa", "aa"]),
            4
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::count_prefix_suffix_pairs(arr!["pa", "papa", "ma", "mama"]),
            2
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::count_prefix_suffix_pairs(arr!["abab", "ab"]), 0);
    }
}
