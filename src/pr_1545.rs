pub struct Solution;

impl Solution {
    pub const fn find_kth_bit(_n: i32, k: i32) -> char {
        let position_in_section = k & -k;
        let is_in_inverted_part = ((k / position_in_section) >> 1 & 1) == 1;
        let original_bit_is_one = (k & 1) == 0;
        if is_in_inverted_part {
            if original_bit_is_one {
                '0'
            } else {
                '1'
            }
        } else if original_bit_is_one {
            '1'
        } else {
            '0'
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::find_kth_bit(3, 1), '0');
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::find_kth_bit(4, 11), '1');
    }
}
