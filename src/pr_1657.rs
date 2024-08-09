pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }
        let (mut map1, mut map2) = (vec![0; 26], vec![0; 26]);
        for (b1, b2) in word1.bytes().zip(word2.bytes()) {
            map1[usize::from(b1 - 97)] += 1;
            map2[usize::from(b2 - 97)] += 1;
        }
        for (cnt1, cnt2) in map1.iter().zip(&map2) {
            if *cnt1 == 0 && *cnt2 != 0 || *cnt1 != 0 && *cnt2 == 0 {
                return false;
            }
        }
        map1.sort_unstable();
        map2.sort_unstable();
        map1 == map2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            true,
            Solution::close_strings("abc".to_owned(), "bca".to_owned())
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            false,
            Solution::close_strings("a".to_owned(), "aa".to_owned())
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            true,
            Solution::close_strings("cabbba".to_owned(), "abbccc".to_owned())
        );
    }

    #[test]
    fn case_4() {
        assert_eq!(
            false,
            Solution::close_strings("dabbba".to_owned(), "abbccc".to_owned())
        );
    }
}
