pub struct Solution;

impl Solution {
    pub const fn is_ugly(mut n: i32) -> bool {
        let (primes, mut i) = (&[2, 3, 5], 0);
        while i < primes.len() {
            while n != 0 && n % primes[i] == 0 {
                n /= primes[i];
            }
            i += 1;
        }
        n == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::is_ugly(6));
    }

    #[test]
    fn case_2() {
        assert!(Solution::is_ugly(1));
    }

    #[test]
    fn case_3() {
        assert!(!Solution::is_ugly(14));
    }
}
