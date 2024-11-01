pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn make_fancy_string(s: String) -> String {
        let mut iter = s.chars();
        let mut prev = iter.next().unwrap();
        let (mut result, mut count) = (String::from(prev), 1);
        for c in iter {
            if c == prev {
                count += 1;
            } else {
                prev = c;
                count = 1;
            }
            if count < 3 {
                result.push(c);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::make_fancy_string("leeetcode".to_owned()),
            "leetcode".to_owned()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::make_fancy_string("aaabaaaa".to_owned()),
            "aabaa".to_owned()
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::make_fancy_string("aab".to_owned()),
            "aab".to_owned()
        );
    }
}
