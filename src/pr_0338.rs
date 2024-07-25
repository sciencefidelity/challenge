pub struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ones = vec![0; usize::try_from(n + 1).unwrap()];
        let mut i = 0;
        while i <= n {
            ones[usize::try_from(i).unwrap()] = Self::get_bits(i);
            i += 1;
        }
        ones
    }

    const fn get_bits(mut x: i32) -> i32 {
        // let mut n = (x >> 1) & 0x7777_7777;
        // x -= n;
        // n = (n >> 1) & 0x7777_7777;
        // x -= n;
        // n = (n >> 1) & 0x7777_7777;
        // x -= n;
        // x = (x + (x >> 4)) & 0x0F0F_0F0F;
        // x *= 0x0101_0101;
        // x >> 24
        x = (x & 0x5555_5555) + ((x >> 1) & 0x5555_5555);
        x = (x & 0x3333_3333) + ((x >> 2) & 0x3333_3333);
        x = (x & 0x0F0F_0F0F) + ((x >> 4) & 0x0F0F_0F0F);
        x = (x & 0x00FF_00FF) + ((x >> 8) & 0x00FF_00FF);
        x = (x & 0x0000_FFFF) + ((x >> 16) & 0x0000_FFFF);
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(vec![0, 1, 1], Solution::count_bits(2));
    }

    #[test]
    fn case_2() {
        assert_eq!(vec![0, 1, 1, 2, 1, 2], Solution::count_bits(5));
    }
}
