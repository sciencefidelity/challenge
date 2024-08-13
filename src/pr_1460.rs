use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut counter = HashMap::with_capacity(arr.len());
        for num in arr {
            counter.entry(num).and_modify(|cnt| *cnt += 1).or_insert(1);
        }
        for num in target {
            match counter.get_mut(&num) {
                Some(cnt) if *cnt > 1 => *cnt -= 1,
                Some(_) => {
                    counter.remove(&num);
                }
                None => return false,
            }
        }
        counter.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3]));
    }

    #[test]
    fn case_2() {
        assert!(Solution::can_be_equal(vec![7], vec![7]));
    }

    #[test]
    fn case_3() {
        assert!(!Solution::can_be_equal(vec![3, 7, 9], vec![3, 7, 11]));
    }
}
