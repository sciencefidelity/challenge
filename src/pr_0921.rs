pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let (mut open, mut close) = (0, 0);
        for b in s.bytes() {
            match b {
                b'(' => open += 1,
                _ if open > 0 => open -= 1,
                _ => close += 1,
            }
        }
        open + close
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::min_add_to_make_valid("())".to_owned()), 1);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::min_add_to_make_valid("(((".to_owned()), 3);
    }
}
