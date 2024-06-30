pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len();
        while l != r {
            let m = (l + r) / 2;
            if nums[m] == target {
                return m as i32;
            } else if nums[m] < target {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }

    #[test]
    fn case_4() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
    }
}
