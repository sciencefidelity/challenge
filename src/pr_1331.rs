use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn array_rank_transform(mut arr: Vec<i32>) -> Vec<i32> {
        let (mut map, mut sorted_arr) = (HashMap::with_capacity(arr.len()), arr.clone());
        sorted_arr.sort_unstable();
        let mut rank = 1;
        for (i, val) in sorted_arr.iter().enumerate() {
            if i > 0 && *val > sorted_arr[i - 1] {
                rank += 1;
            }
            map.insert(val, rank);
        }
        for val in &mut arr {
            *val = *map.get(val).unwrap();
        }
        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::array_rank_transform(vec![40, 10, 20, 30]),
            vec![4, 1, 2, 3]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::array_rank_transform(vec![100, 100, 100]),
            vec![1, 1, 1]
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
            vec![5, 3, 4, 2, 8, 6, 7, 1, 3]
        );
    }
}
