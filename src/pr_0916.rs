use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut bmax = [0; 26];
        for b in words2 {
            let curr_freq = Self::count(&b);
            for i in 0..26 {
                bmax[i] = max(bmax[i], curr_freq[i]);
            }
        }
        words1
            .into_iter()
            .filter(|word| {
                let word_freq = Self::count(word);
                (0..26).all(|i| word_freq[i] >= bmax[i])
            })
            .collect()
    }

    fn count(word: &str) -> Vec<i32> {
        let mut freq = vec![0; 26];
        for b in word.bytes() {
            freq[usize::from(b - b'a')] += 1;
        }
        freq
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::word_subsets(
                arr!["amazon", "apple", "facebook", "google", "leetcode"],
                arr!["e", "o"]
            ),
            arr!["facebook", "google", "leetcode"]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::word_subsets(
                arr!["amazon", "apple", "facebook", "google", "leetcode"],
                arr!["l", "e"]
            ),
            arr!["apple", "google", "leetcode"]
        );
    }
}
