#![allow(clippy::cast_sign_loss)]
pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut prefix_common_array = vec![0; n];
        let mut frequency = vec![0; n + 1];
        let mut common_count = 0;
        for current_index in 0..n {
            frequency[a[current_index] as usize] += 1;
            if frequency[a[current_index] as usize] == 2 {
                common_count += 1;
            }
            frequency[b[current_index] as usize] += 1;
            if frequency[b[current_index] as usize] == 2 {
                common_count += 1;
            }
            prefix_common_array[current_index] = common_count;
        }
        prefix_common_array
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::find_the_prefix_common_array(vec![1, 3, 2, 4], vec![3, 1, 2, 4]),
            vec![0, 2, 3, 4]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::find_the_prefix_common_array(vec![2, 3, 1], vec![3, 1, 2]),
            vec![0, 1, 3]
        );
    }
}
