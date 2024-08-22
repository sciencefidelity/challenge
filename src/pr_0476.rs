pub struct Solution;

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        !num & i32::try_from((u32::try_from(num).unwrap().next_power_of_two() - 1).max(1)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::find_complement(5), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::find_complement(1), 0);
    }
}
