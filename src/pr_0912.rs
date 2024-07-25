pub struct Solution;

pub trait Sorting {
    fn mergesort(&mut self);
    fn quicksort(&mut self);
}

impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        nums.mergesort();
        nums
    }
}

impl<T: Ord> Sorting for [T] {
    fn mergesort(&mut self) {
        let n = self.len();
        let mut width = 1;
        while width < n {
            let mut i = 0;
            while i < n {
                let left = i;
                let mid = i + width;
                let right = if i + 2 * width > n { n } else { i + 2 * width };
                in_place_merge(self, left, mid, right);
                i += 2 * width;
            }
            width *= 2;
        }
    }

    fn quicksort(&mut self) {
        if self.is_empty() {
            return;
        }
        let n = self.len();
        Quicksort::quicksort(self, 0, n - 1);
    }
}

fn in_place_merge<T: Ord>(arr: &mut [T], left: usize, mid: usize, right: usize) {
    let mut i = left;
    let mut j = mid;
    while i < j && j < right {
        if arr[i] > arr[j] {
            j += 1;
            arr[i..j].rotate_right(1);
        }
        i += 1;
    }
}

pub struct Quicksort;

impl Quicksort {
    fn quicksort<T: Ord>(arr: &mut [T], low: usize, high: usize) {
        if low >= high {
            return;
        }
        let pivot_index = Self::partition(arr, low, high);
        if pivot_index > 0 {
            Self::quicksort(arr, low, pivot_index - 1);
        }
        if pivot_index < arr.len() {
            Self::quicksort(arr, pivot_index + 1, high);
        }
    }

    fn partition<T: Ord>(arr: &mut [T], low: usize, high: usize) -> usize {
        let pivot = high;
        let mut i = low;
        for j in low..high {
            if arr[j] <= arr[pivot] {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, high);
        i
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(vec![1, 2, 3, 5], Solution::sort_array(vec![5, 2, 3, 1]));
    }

    #[test]
    fn case_2() {
        assert_eq!(
            vec![0, 0, 1, 1, 2, 5],
            Solution::sort_array(vec![5, 1, 1, 2, 0, 0])
        );
    }
}
