#![allow(clippy::cast_possible_wrap)]
pub struct Solution;

impl Solution {
    pub const fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let mut result = 0;
        let target_set_bits_count = num2.count_ones() as i32;
        let mut set_bits_count = 0;
        let mut current_bit = 31;
        while set_bits_count < target_set_bits_count {
            if Self::is_set(num1, current_bit)
                || (target_set_bits_count - set_bits_count > current_bit)
            {
                result = Self::set_bit(result, current_bit);
                set_bits_count += 1;
            }
            current_bit -= 1;
        }
        result
    }

    const fn is_set(x: i32, bit: i32) -> bool {
        x & (1 << bit) != 0
    }

    const fn set_bit(x: i32, bit: i32) -> i32 {
        x | (1 << bit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::minimize_xor(3, 5), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::minimize_xor(1, 5), 3);
    }
}
