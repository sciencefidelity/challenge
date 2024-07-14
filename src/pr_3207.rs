use std::cmp::min;

pub struct Solution;

impl Solution {
    pub fn maximum_points(enemy_energies: Vec<i32>, current_energy: i32) -> i64 {
        let current_energy = i64::from(current_energy);
        let mut min_energy = i64::MAX;
        let mut points = 0_i64;
        for enemy_energy in enemy_energies {
            min_energy = min(min_energy, i64::from(enemy_energy));
            points += i64::from(enemy_energy);
        }
        if current_energy < min_energy {
            return 0;
        }
        points -= min_energy;
        current_energy / min_energy + current_energy % min_energy + points / min_energy
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
