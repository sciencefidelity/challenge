// use std::collections::BinaryHeap;

pub trait Select<T> {
    fn quickselect(&mut self, k: usize) -> T;
}

pub struct Solution;

impl Solution {
    // pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    //     let mut heap = BinaryHeap::with_capacity(nums.len());
    //     for num in nums {
    //         heap.push(num);
    //     }
    //     let mut output = 0;
    //     for _ in 0..k {
    //         output = heap.pop().unwrap();
    //     }
    //     output
    // }

    // pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
    //     let (n, k) = (nums.len(), usize::try_from(k).unwrap());
    //     *nums.select_nth_unstable(n - k).1
    // }

    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let (n, k) = (nums.len(), usize::try_from(k).unwrap());
        let out = nums.quickselect(n - k);
        out
    }
}

impl<T: Ord + Copy> Select<T> for [T] {
    fn quickselect(&mut self, k: usize) -> T {
        let n = self.len();
        Quickselect::quickselect(self, k, 0, n - 1)
    }
}

struct Quickselect;

impl Quickselect {
    fn quickselect<T: Ord + Copy>(
        list: &mut [T],
        kth_lowset_value: usize,
        left_index: usize,
        right_index: usize,
    ) -> T {
        if right_index - left_index <= 0 {
            return list[left_index];
        }
        let pivot_index = Self::partition(list, left_index, right_index);

        if kth_lowset_value < pivot_index {
            Self::quickselect(list, kth_lowset_value, left_index, pivot_index - 1)
        } else if kth_lowset_value > pivot_index {
            Self::quickselect(list, kth_lowset_value, pivot_index + 1, right_index)
        } else {
            list[pivot_index]
        }
    }

    fn partition<T: Ord + Copy>(
        list: &mut [T],
        mut left_pointer: usize,
        mut right_pointer: usize,
    ) -> usize {
        let pivot_index = right_pointer;
        let pivot = list[pivot_index];
        right_pointer -= 1;
        loop {
            while list[left_pointer] < pivot {
                left_pointer += 1;
            }
            while list[right_pointer] > pivot {
                right_pointer -= 1;
            }
            if left_pointer >= right_pointer {
                break;
            } else {
                list.swap(left_pointer, right_pointer);
                left_pointer += 1;
            }
        }
        list.swap(left_pointer, pivot_index);
        left_pointer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(5, Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2));
    }

    #[test]
    fn case_2() {
        assert_eq!(
            4,
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4)
        );
    }
}
