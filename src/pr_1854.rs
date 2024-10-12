#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]
pub struct Solution;

impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut years = vec![0; 101];
        for log in logs {
            years[log[0] as usize - 1950] += 1;
            years[log[1] as usize - 1950] -= 1;
        }
        let (mut max_num, mut max_year) = (years[0], 1950);
        for i in 1..years.len() {
            years[i] += years[i - 1];
            if years[i] > max_num {
                max_num = years[i];
                max_year = i + 1950;
            }
        }
        max_year as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::maximum_population(arr![[1993, 1999], [2000, 2010]]),
            1993
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::maximum_population(arr![[1950, 1961], [1960, 1971], [1970, 1981]]),
            1960
        );
    }
}
