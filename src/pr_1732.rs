pub struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut net_altitude = 0;
        let mut highest_altitude = 0;
        for i in gain {
            net_altitude += i;
            highest_altitude = highest_altitude.max(net_altitude);
        }
        highest_altitude
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::largest_altitude(vec![-5, 1, 5, 0, -7]), 1);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]), 0);
    }
}
