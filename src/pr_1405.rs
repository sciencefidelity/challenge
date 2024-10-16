pub struct Solution;

impl Solution {
    pub fn longest_diverse_string(mut a: i32, mut b: i32, mut c: i32) -> String {
        let (mut curr_a, mut curr_b, mut curr_c) = (0, 0, 0);
        let (total_iterations, mut result) = (a + b + c, String::new());
        for _ in 0..total_iterations {
            if (a >= b && a >= c && curr_a != 2) || (a > 0 && (curr_b == 2 || curr_c == 2)) {
                result.push('a');
                a -= 1;
                curr_a += 1;
                curr_b = 0;
                curr_c = 0;
            } else if (b >= a && b >= c && curr_b != 2) || (b > 0 && (curr_c == 2 || curr_a == 2)) {
                result.push('b');
                b -= 1;
                curr_b += 1;
                curr_a = 0;
                curr_c = 0;
            } else if (c >= a && c >= b && curr_c != 2) || (c > 0 && (curr_a == 2 || curr_b == 2)) {
                result.push('c');
                c -= 1;
                curr_c += 1;
                curr_a = 0;
                curr_b = 0;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::longest_diverse_string(1, 1, 7), "ccaccbcc");
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::longest_diverse_string(7, 1, 0), "aabaa");
    }
}
