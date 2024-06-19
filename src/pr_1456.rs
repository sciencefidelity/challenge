pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let k = k as usize;
        let mut queue = VecDeque::new();
        let mut count = 0;
        let mut max = 0;

        for c in s.chars() {
            if queue.len() == k {
                let a = queue.pop_front().unwrap();
                if Self::is_vowel(a) {
                    count -= 1;
                }
            }
            if Self::is_vowel(c) {
                count += 1;
                max = max.max(count);
            }
            queue.push_back(c);
        }
        max
    }

    fn is_vowel(c: char) -> bool {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::max_vowels("abciiidef".to_owned(), 3), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::max_vowels("aeiou".to_owned(), 2), 2);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::max_vowels("leetcode".to_owned(), 3), 2);
    }

    #[test]
    fn case_4() {
        assert_eq!(Solution::max_vowels("ramadan".to_owned(), 2), 1);
    }

    #[test]
    fn case_5() {
        assert_eq!(Solution::max_vowels("weallloveyou".to_owned(), 7), 4);
    }
}
