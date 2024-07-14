pub struct Solution;

use std::cmp::Ordering::{Equal, Greater, Less};

impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        mut healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let directions = directions.as_bytes();
        let mut robots = (0..positions.len()).collect::<Vec<_>>();

        robots.sort_unstable_by_key(|&i| positions[i]);
        let mut survivors = Vec::new();

        for robot_a in robots {
            if directions[robot_a] == b'R' {
                survivors.push(robot_a);
            } else {
                while let Some(&robot_b) = survivors.last() {
                    if directions[robot_b] == b'R' && healths[robot_a] > 0 {
                        match healths[robot_a].cmp(&healths[robot_b]) {
                            Equal => {
                                survivors.pop();
                                healths[robot_a] = 0;
                            }
                            Less => {
                                healths[robot_a] = 0;
                                healths[robot_b] -= 1;
                            }
                            Greater => {
                                survivors.pop();
                                healths[robot_a] -= 1;
                            }
                        }
                    } else {
                        break;
                    }
                }
                if healths[robot_a] > 0 {
                    survivors.push(robot_a);
                }
            }
        }
        survivors.sort_unstable();
        survivors.into_iter().map(|i| healths[i]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::survived_robots_healths(
                vec![5, 4, 3, 2, 1],
                vec![2, 17, 9, 15, 10],
                "RRRRR".into()
            ),
            vec![2, 17, 9, 15, 10]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::survived_robots_healths(
                vec![3, 5, 2, 6],
                vec![10, 10, 15, 12],
                "RLRL".into()
            ),
            vec![14]
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::survived_robots_healths(
                vec![1, 2, 5, 6],
                vec![10, 10, 11, 11],
                "RLRL".into()
            ),
            vec![]
        );
    }
}
