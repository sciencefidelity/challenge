pub struct Solution;

impl Solution {
    pub const fn is_happy(n: i32) -> bool {
        let mut slow = n;
        let mut fast = n;
        loop {
            slow = Self::square(slow);
            fast = Self::square(Self::square(fast));
            if slow == 1 {
                return true;
            }
            if slow == fast {
                return false;
            }
        }
    }

    const fn square(mut num: i32) -> i32 {
        let mut square = 0;
        while num > 0 {
            let remainder = num % 10;
            square += remainder * remainder;
            num /= 10;
        }
        square
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::is_happy(19), true);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::is_happy(2), false);
    }
}
