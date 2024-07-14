pub struct Solution;

impl Solution {
    pub const fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let full_rounds = time / (n - 1);
        let extra_time = time % (n - 1);
        if full_rounds % 2 == 0 {
            extra_time + 1
        } else {
            n - extra_time
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::pass_the_pillow(4, 5), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::pass_the_pillow(3, 2), 3);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::pass_the_pillow(9, 4), 5);
    }
}
