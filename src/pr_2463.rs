#![allow(clippy::cast_possible_truncation)]
pub struct Solution;

impl Solution {
    pub fn minimum_total_distance(robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {
        let mut robot = robot;
        robot.sort_unstable();
        let mut factory = factory;
        factory.sort_unstable_by_key(|f| f[0]);
        let mut factory_positions = Vec::new();
        for f in &factory {
            for _ in 0..f[1] {
                factory_positions.push(f[0]);
            }
        }
        let robot_count = robot.len();
        let factory_count = factory_positions.len();
        let mut dp = vec![vec![0i64; factory_count + 1]; robot_count + 1];
        for item in dp.iter_mut().take(robot_count) {
            item[factory_count] = 1e12 as i64;
        }
        for i in (0..robot_count).rev() {
            for j in (0..factory_count).rev() {
                let assign = i64::from((robot[i] - factory_positions[j]).abs()) + dp[i + 1][j + 1];
                let skip = dp[i][j + 1];
                dp[i][j] = std::cmp::min(assign, skip);
            }
        }
        dp[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::minimum_total_distance(vec![0, 4, 6], arr![[2, 2], [6, 2]]),
            4
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::minimum_total_distance(vec![1, -1], arr![[-2, 1], [2, 1]]),
            2
        );
    }
}
