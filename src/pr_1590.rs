use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let (n, mut total_sum) = (i32::try_from(nums.len()).unwrap(), 0);
        for num in &nums {
            total_sum = (total_sum + num) % p;
        }
        let target = total_sum % p;
        if target == 0 {
            return 0;
        }
        let mut mod_map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        mod_map.insert(0, -1);
        let (mut current_sum, mut min_len, mut i) = (0, n, 0);
        for num in nums {
            current_sum = (current_sum + num) % p;
            let needed = (current_sum - target + p) % p;
            if mod_map.contains_key(&needed) {
                min_len = min_len.min(i - *mod_map.get(&needed).unwrap());
            }
            mod_map.insert(current_sum, i);
            i += 1;
        }
        if min_len == n {
            -1
        } else {
            min_len
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::min_subarray(vec![3, 1, 4, 2], 6), 1);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::min_subarray(vec![6, 3, 5, 2], 9), 2);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::min_subarray(vec![1, 2, 3], 3), 0);
    }
}
