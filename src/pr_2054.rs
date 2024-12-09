#![allow(clippy::type_complexity, clippy::needless_pass_by_value)]
pub struct Solution;

impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let (mut starts, mut ends): (Vec<(i32, i32)>, Vec<(i32, i32)>) = events
            .into_iter()
            .map(|event| ((event[0], event[2]), (event[1], event[2])))
            .unzip();

        starts.sort_unstable();
        ends.sort_unstable();
        let mut first_end_idx = 0;
        let mut cur_first_max = 0;
        let mut total_max = 0;
        for (second_start, second_val) in starts {
            while ends[first_end_idx].0 < second_start {
                cur_first_max = cur_first_max.max(ends[first_end_idx].1);
                first_end_idx += 1;
            }
            total_max = total_max.max(cur_first_max + second_val);
        }
        total_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::max_two_events(arr![[1, 3, 2], [4, 5, 2], [2, 4, 3]]),
            4
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::max_two_events(arr![[1, 3, 2], [4, 5, 2], [1, 5, 5]]),
            5
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::max_two_events(arr![[1, 5, 3], [1, 5, 1], [6, 6, 5]]),
            8
        );
    }
}
