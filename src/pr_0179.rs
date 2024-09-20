pub struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums: Vec<String> = nums.into_iter().map(|i| i.to_string()).collect();
        nums.sort_unstable_by(|a, b| b.bytes().chain(a.bytes()).cmp(a.bytes().chain(b.bytes())));
        if nums[0].starts_with('0') {
            String::from("0")
        } else {
            nums.into_iter().collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::largest_number(vec![10, 2]), "210".to_owned());
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::largest_number(vec![3, 30, 34, 5, 9]),
            "9534330".to_owned()
        );
    }
}
