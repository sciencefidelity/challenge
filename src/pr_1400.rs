pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn can_construct(s: String, k: i32) -> bool {
        let k = usize::try_from(k).unwrap();
        if s.len() < k {
            return false;
        }
        if s.len() == k {
            return true;
        }
        let mut odd_count = 0i32;
        for b in s.bytes() {
            odd_count ^= 1 << (b - b'a');
        }
        let set_bits = odd_count.count_ones();
        set_bits <= u32::try_from(k).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::can_construct("annabelle".to_owned(), 2));
    }

    #[test]
    fn case_2() {
        assert!(!Solution::can_construct("leetcode".to_owned(), 3));
    }

    #[test]
    fn case_3() {
        assert!(Solution::can_construct("true".to_owned(), 4));
    }
}
