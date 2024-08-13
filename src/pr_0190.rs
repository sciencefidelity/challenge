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
        assert_eq!(Solution::reverse_bits(43_261_596), 964_176_192);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::reverse_bits(4_294_967_293), 3_221_225_471);
    }
}
