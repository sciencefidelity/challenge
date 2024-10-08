pub struct Solution;

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut stack_size = 0;
        for b in s.into_bytes() {
            if b == b'[' {
                stack_size += 1;
            } else if stack_size > 0 {
                stack_size -= 1;
            }
        }
        (stack_size + 1) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::min_swaps("][][".to_owned()), 1);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::min_swaps("]]][[[".to_owned()), 2);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::min_swaps("[]".to_owned()), 0);
    }
}
