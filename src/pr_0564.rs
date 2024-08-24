pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn nearest_palindromic(s: String) -> String {
        let (num, len) = (s.parse::<i64>().unwrap(), s.len());
        let half = s[..(len + 1) / 2].parse::<i64>().unwrap();
        let mut candidates = vec![
            10_i64.pow(u32::try_from(len).unwrap()) + 1,
            10_i64.pow(u32::try_from(len - 1).unwrap()) - 1,
        ];
        for &i in &[half - 1, half, half + 1] {
            let mut cand_str = i.to_string();
            let rev: String = cand_str.chars().rev().collect();
            if len % 2 != 0 {
                cand_str.pop();
            }
            cand_str.push_str(&rev);
            candidates.push(cand_str.parse().unwrap());
        }
        candidates
            .into_iter()
            .filter(|&c| c != num)
            .min_by_key(|&c| ((c - num).abs(), c))
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::nearest_palindromic("123".to_owned()),
            "121".to_owned()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::nearest_palindromic("1".to_owned()),
            "0".to_owned()
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::nearest_palindromic("1234".to_owned()),
            "1221".to_owned()
        );
    }

    #[test]
    fn case_4() {
        assert_eq!(
            Solution::nearest_palindromic("999".to_owned()),
            "1001".to_owned()
        );
    }

    #[test]
    fn case_5() {
        assert_eq!(
            Solution::nearest_palindromic("1000".to_owned()),
            "999".to_owned()
        );
    }

    #[test]
    fn case_6() {
        assert_eq!(
            Solution::nearest_palindromic("12932".to_owned()),
            "12921".to_owned()
        );
    }

    #[test]
    fn case_7() {
        assert_eq!(
            Solution::nearest_palindromic("99800".to_owned()),
            "99799".to_owned()
        );
    }

    #[test]
    fn case_8() {
        assert_eq!(
            Solution::nearest_palindromic("12120".to_owned()),
            "12121".to_owned()
        );
    }

    #[test]
    fn case_9() {
        assert_eq!(
            Solution::nearest_palindromic("12345".to_owned()),
            "12321".to_owned()
        );
    }
}
