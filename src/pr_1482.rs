pub struct Solution;

impl Solution {
    fn get_num_of_bouquets(bloom_day: &[i32], mid: i32, k: i32) -> i32 {
        let mut num_of_bouquets = 0;
        let mut count = 0;

        for day in bloom_day {
            if *day <= mid {
                count += 1;
            } else {
                count = 0;
            }

            if count == k {
                num_of_bouquets += 1;
                count = 0;
            }
        }
        num_of_bouquets
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        let mut start = 0;
        let mut end = 0;
        for day in &bloom_day {
            end = end.max(*day);
        }

        let mut min_days = -1;
        while start <= end {
            let mid = (start + end) / 2;

            if Self::get_num_of_bouquets(&bloom_day, mid, k) >= m {
                min_days = mid;
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }
        min_days
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::min_days(vec![1, 10, 3, 10, 2], 3, 1), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::min_days(vec![1, 10, 3, 10, 2], 3, 2), -1);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::min_days(vec![7, 7, 7, 7, 12, 7, 7], 2, 3), 12);
    }
}
