pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
        let mut groups = 0;
        let first = colors[0];
        let last = colors[colors.len() - 1];
        for (i, color) in colors.iter().enumerate() {
            let before = if i == 0 { last } else { colors[i - 1] };
            let after = if i == colors.len() - 1 {
                first
            } else {
                colors[i + 1]
            };
            if before != *color && after != *color {
                groups += 1;
            }
        }

        groups
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::number_of_alternating_groups(vec![1, 1, 1]), 0);
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::number_of_alternating_groups(vec![0, 1, 0, 0, 1]),
            3
        );
    }
}
