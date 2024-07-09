pub struct Solution;

impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut reversed = 0u32;
        for _ in 0..32 {
            reversed = (reversed << 1) | (x & 1);
            x >>= 1;
        }
        reversed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::reverse_bits(43261596), 964176192);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::reverse_bits(4294967293), 3221225471);
    }
}
