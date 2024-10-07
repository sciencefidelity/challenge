pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn min_length(s: String) -> i32 {
        let mut stack = Vec::with_capacity(s.len());
        for b in s.bytes() {
            match (stack.last(), b) {
                (Some(b'A'), b'B') | (Some(b'C'), b'D') => {
                    stack.pop();
                }
                _ => stack.push(b),
            }
        }
        i32::try_from(stack.len()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::min_length("ABFCACDB".to_owned()), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::min_length("ACBBD".to_owned()), 5);
    }
}
