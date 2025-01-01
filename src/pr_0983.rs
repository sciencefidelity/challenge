#![allow(clippy::cast_sign_loss)]
pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let last_day = days[days.len() - 1];
        let mut dp = vec![0; (last_day + 1) as usize];
        let mut i = 0;
        for day in 1..=last_day {
            if day < days[i] {
                dp[day as usize] = dp[(day - 1) as usize];
            } else {
                i += 1;
                dp[day as usize] = *[
                    dp[(day - 1) as usize] + costs[0],
                    dp[0.max(day - 7) as usize] + costs[1],
                    dp[0.max(day - 30) as usize] + costs[2],
                ]
                .iter()
                .min()
                .unwrap();
            }
        }
        dp[last_day as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]),
            11
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15]),
            17
        );
    }
}
