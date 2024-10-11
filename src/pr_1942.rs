use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn smallest_chair(mut times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let n = times.len();
        let target = times[usize::try_from(target_friend).unwrap()][0];
        let mut chairs = BinaryHeap::with_capacity(n);
        let mut depart: BinaryHeap<(i32, i32)> = BinaryHeap::with_capacity(n);
        let mut next_chair = 0;
        times.sort_unstable();
        for pair in times {
            let (arrive, leave) = (pair[0], pair[1]);
            while let Some(&(leaving, chair)) = depart.peek() {
                if arrive < -leaving {
                    break;
                }
                depart.pop();
                chairs.push(-chair);
            }
            if chairs.is_empty() {
                chairs.push(-next_chair);
                next_chair += 1;
            }
            let chair = -chairs.pop().unwrap();
            if arrive == target {
                return chair;
            }
            depart.push((-leave, chair));
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(Solution::smallest_chair(arr![[1, 4], [2, 3], [4, 6]], 1), 1);
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::smallest_chair(arr![[3, 10], [1, 5], [2, 6]], 0),
            2
        );
    }
}
