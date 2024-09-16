pub struct Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut minutes = vec![false; 24 * 60];
        for t in time_points {
            let min = Self::clock_time_to_minutes(&t);
            if minutes[min] {
                return 0;
            }
            minutes[min] = true;
        }
        let (mut prev_idx, mut first_idx, mut last_idx, mut result) =
            (usize::MAX, usize::MAX, usize::MAX, usize::MAX);
        for (i, min) in minutes.into_iter().enumerate() {
            if min {
                if prev_idx != usize::MAX {
                    result = result.min(i - prev_idx);
                }
                prev_idx = i;
                if first_idx == usize::MAX {
                    first_idx = i;
                }
                last_idx = i;
            }
        }
        result
            .min(24 * 60 - last_idx + first_idx)
            .try_into()
            .unwrap()
    }

    fn clock_time_to_minutes(time: &str) -> usize {
        let (h, m) = (
            &time[..2].parse::<usize>().unwrap(),
            &time[3..].parse::<usize>().unwrap(),
        );
        h * 60 + m
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(Solution::find_min_difference(arr!["23:59", "00:00"]), 1);
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::find_min_difference(arr!["00:00", "23:59", "00:00"]),
            0
        );
    }
}
