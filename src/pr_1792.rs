#![allow(
    clippy::unreadable_literal,
    clippy::derive_ord_xor_partial_ord,
    clippy::cast_lossless,
    clippy::cast_precision_loss,
    clippy::float_cmp
)]
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, PartialOrd, Debug)]
struct Wrapper(f64);

impl Ord for Wrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl Eq for Wrapper {}

pub struct Solution;

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let n = classes.len();
        let mut heap = BinaryHeap::with_capacity(n);
        for class in classes {
            heap.push((Self::calculate_gain(class[0], class[1]), class[0], class[1]));
        }
        for _ in 0..extra_students {
            let (_, passes, total_students) = heap.pop().unwrap();
            heap.push((
                Self::calculate_gain(passes + 1, total_students + 1),
                passes + 1,
                total_students + 1,
            ));
        }
        let mut total_pass_ratio = 0.0;
        while let Some((_, passes, total_students)) = heap.pop() {
            total_pass_ratio += passes as f64 / total_students as f64;
        }
        total_pass_ratio / n as f64
    }

    fn calculate_gain(passes: i32, total_students: i32) -> Wrapper {
        Wrapper(
            (passes + 1) as f64 / (total_students + 1) as f64
                - passes as f64 / total_students as f64,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::max_average_ratio(arr![[1, 2], [3, 5], [2, 2]], 2),
            0.7833333333333333
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::max_average_ratio(arr![[2, 4], [3, 9], [4, 5], [2, 10]], 4),
            0.5348484848484849
        );
    }
}
