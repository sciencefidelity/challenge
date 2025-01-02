#![allow(clippy::cast_sign_loss)]
pub struct Solution;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prefix_sum = vec![0; words.len() + 1];
        for (i, word) in words.into_iter().enumerate() {
            prefix_sum[i + 1] = prefix_sum[i]
                + i32::from(
                    Self::is_vowel(word.as_bytes()[0])
                        && Self::is_vowel(word.as_bytes()[word.len() - 1]),
                );
        }
        queries
            .into_iter()
            .map(|q| prefix_sum[q[1] as usize + 1] - prefix_sum[q[0] as usize])
            .collect()
    }

    const fn is_vowel(b: u8) -> bool {
        matches!(b, b'a' | b'e' | b'i' | b'o' | b'u')
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::vowel_strings(
                arr!["aba", "bcb", "ece", "aa", "e"],
                arr![[0, 2], [1, 4], [1, 1]]
            ),
            vec![2, 3, 0]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::vowel_strings(arr!["a", "e", "i"], arr![[0, 2], [0, 1], [2, 2]]),
            vec![3, 2, 1]
        );
    }
}
