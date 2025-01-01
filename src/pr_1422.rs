pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn max_score(s: String) -> i32 {
        let (mut ones, mut zeros, mut best, n) = (0, 0, i32::MIN, s.len());
        let s = s.as_bytes();
        for b in s.iter().take(n - 1) {
            if *b == b'1' {
                ones += 1;
            } else {
                zeros += 1;
            }
            best = best.max(zeros - ones);
        }
        if s[n - 1] == b'1' {
            ones += 1;
        }
        best + ones
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::max_score("011101".to_owned()), 5);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::max_score("00111".to_owned()), 5);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::max_score("1111".to_owned()), 3);
    }
}
