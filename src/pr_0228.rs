pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut ranges = Vec::new();
        let mut i = 0;
        while i < nums.len() {
            let start = nums[i];
            while i + 1 < nums.len() && nums[i + 1] == nums[i] + 1 {
                i += 1;
            }
            let end = nums[i];
            if start == end {
                ranges.push(start.to_string());
            } else {
                ranges.push(format!("{start}->{end}"));
            }
            i += 1;
        }
        ranges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec!["0->2".to_owned(), "4->5".to_owned(), "7".to_owned()]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
            vec![
                "0".to_owned(),
                "2->4".to_owned(),
                "6".to_owned(),
                "8->9".to_owned()
            ]
        );
    }
}
