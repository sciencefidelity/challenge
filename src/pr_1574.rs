pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let mut right = arr.len() - 1;
        while right > 0 && arr[right] >= arr[right - 1] {
            right -= 1;
        }
        let (mut ans, mut left) = (right, 0);
        while left < right && (left == 0 || arr[left - 1] <= arr[left]) {
            while right < arr.len() && arr[left] > arr[right] {
                right += 1;
            }
            ans = ans.min(right - left - 1);
            left += 1;
        }
        i32::try_from(ans).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::find_length_of_shortest_subarray(vec![1, 2, 3, 10, 4, 2, 3, 5]),
            3
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::find_length_of_shortest_subarray(vec![5, 4, 3, 2, 1]),
            4
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::find_length_of_shortest_subarray(vec![1, 2, 3]), 0);
    }
}
