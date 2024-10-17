pub struct Solution;

impl Solution {
    pub fn maximum_swap(mut num: i32) -> i32 {
        let (res, mut diff) = (num, 0);
        let (mut max_digit, mut max_digit_pos, mut top_digit_pos) = (-1, 0, 1);
        while num > 0 {
            let cur_digit = num % 10;
            if cur_digit > max_digit {
                max_digit = cur_digit;
                max_digit_pos = top_digit_pos;
            } else {
                diff = diff.max(
                    max_digit * (top_digit_pos - max_digit_pos)
                        - cur_digit * (top_digit_pos - max_digit_pos),
                );
            }
            top_digit_pos *= 10;
            num /= 10;
        }
        res + diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::maximum_swap(2736), 7236);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::maximum_swap(9973), 9973);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::maximum_swap(9379), 9973);
    }
}
