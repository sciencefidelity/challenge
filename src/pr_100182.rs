pub struct Solution;

impl Solution {
    pub fn maximum_points(mut enemy_energies: Vec<i32>, mut current_energy: i32) -> i64 {
        enemy_energies.sort_unstable();
        let mut points = 0;
        loop {
            let first = enemy_energies[0].clone();
            let n = enemy_energies.len();
            if current_energy >= first {
                points += 1;
                current_energy -= first;
            } else if points >= 1 {
                current_energy += enemy_energies.pop().unwrap();
            } else {
                return points;
            }
            if current_energy < first && n <= 1 {
                return points;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::maximum_points(vec![3, 2, 2], 2), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::maximum_points(vec![2], 10), 5);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::maximum_points(vec![1, 2, 3], 0), 0);
    }
}
