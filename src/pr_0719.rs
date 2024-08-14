pub struct Solution;

impl Solution {
    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let (k, n) = (usize::try_from(k).unwrap(), nums.len());
        let (mut low, mut high) = (0, nums[n - 1] - nums[0]);
        while low < high {
            let mid = (low + high) / 2;
            let count = Self::count_pairs_whit_max_distance(&nums, mid);
            if count < k {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low
    }

    fn count_pairs_whit_max_distance(nums: &[i32], max_distance: i32) -> usize {
        let (n, mut count, mut left) = (nums.len(), 0, 0);
        for right in 0..n {
            while nums[right] - nums[left] > max_distance {
                left += 1;
            }
            count += right - left;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(0, Solution::smallest_distance_pair(vec![1, 3, 1], 1));
    }

    #[test]
    fn case_2() {
        assert_eq!(0, Solution::smallest_distance_pair(vec![1, 1, 1], 2));
    }

    #[test]
    fn case_3() {
        assert_eq!(5, Solution::smallest_distance_pair(vec![1, 6, 1], 3));
    }
}
