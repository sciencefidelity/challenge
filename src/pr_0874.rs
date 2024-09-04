use std::collections::HashSet;

pub struct Solution;

const HASH_MULTIPLIER: i32 = 60001;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let obstacle_set: HashSet<i32> = obstacles
            .into_iter()
            .map(|obstacle| Self::hash_coordinates(obstacle[0], obstacle[1]))
            .collect();
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut current_position = (0, 0);
        let (mut max_distance_squared, mut current_direction) = (0, 0);
        for command in commands {
            match command {
                -1 => {
                    current_direction = (current_direction + 1) % 4;
                    continue;
                }
                -2 => {
                    current_direction = (current_direction + 3) % 4;
                    continue;
                }
                _ => {}
            }

            let direction = directions[current_direction];
            for _ in 0..command {
                let next_x = current_position.0 + direction.0;
                let next_y = current_position.1 + direction.1;
                if obstacle_set.contains(&Self::hash_coordinates(next_x, next_y)) {
                    break;
                }
                current_position = (next_x, next_y);
                max_distance_squared = max_distance_squared.max(
                    current_position.0 * current_position.0
                        + current_position.1 * current_position.1,
                );
            }
        }
        max_distance_squared
    }

    const fn hash_coordinates(x: i32, y: i32) -> i32 {
        x + HASH_MULTIPLIER * y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(Solution::robot_sim(vec![4, -1, 3], Vec::new()), 25);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::robot_sim(vec![4, -1, 4, -2, 4], arr![[2, 4]]), 65);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::robot_sim(vec![6, -1, -1, 6], Vec::new()), 36);
    }
}
