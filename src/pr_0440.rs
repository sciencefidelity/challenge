#![allow(clippy::cast_possible_truncation)]
pub struct Solution;

impl Solution {
    pub const fn find_kth_number(n: i32, mut k: i32) -> i32 {
        let mut curr = 1;
        k -= 1;
        while k > 0 {
            let step = count_steps(n, curr, curr + 1);
            if step <= k {
                curr += 1;
                k -= step;
            } else {
                curr *= 10;
                k -= 1;
            }
        }
        curr as i32
    }
}

const fn count_steps(n: i32, mut prefix1: i64, mut prefix2: i64) -> i32 {
    let (n, mut steps) = (n as i64, 0);
    while prefix1 <= n {
        steps += if n + 1 < prefix2 { n + 1 } else { prefix2 } - prefix1;
        prefix1 *= 10;
        prefix2 *= 10;
    }
    steps as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::find_kth_number(13, 2), 10);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::find_kth_number(1, 1), 1);
    }
}
