pub struct Solution;

impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        let mut answer = 0;
        let n = position.len();
        position.sort_unstable();

        let mut low = 1;
        let mut high = position[n - 1] / (m - 1);
        while low <= high {
            let mid = low + (high - low) / 2;

            if Self::can_place_balls(mid, &position, m) {
                answer = mid;
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        answer
    }

    const fn can_place_balls(x: i32, position: &[i32], m: i32) -> bool {
        let mut prev_ball_pos = position[0];
        let mut balls_placed = 1;
        let mut i = 1;
        while i < position.len() && balls_placed < m {
            let curr_pos = position[i];
            if curr_pos - prev_ball_pos >= x {
                balls_placed += 1;
                prev_ball_pos = curr_pos;
            }
            i += 1;
        }
        balls_placed == m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::max_distance(vec![1, 2, 3, 4, 7], 3), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::max_distance(vec![5, 4, 3, 2, 1, 1000000000], 2),
            999999999
        );
    }
}
