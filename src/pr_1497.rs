pub struct Solution;

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut mod_counts = vec![0; usize::try_from(k).unwrap()];
        for num in arr {
            mod_counts[usize::try_from(num.rem_euclid(k)).unwrap()] += 1;
        }
        if mod_counts[0] & 1 == 1 {
            return false;
        }
        for rem in 1..(k + 1) / 2 {
            if mod_counts[usize::try_from(rem).unwrap()]
                != mod_counts[usize::try_from(k - rem).unwrap()]
            {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::can_arrange(
            vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9],
            5
        ));
    }

    #[test]
    fn case_2() {
        assert!(Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 7));
    }

    #[test]
    fn case_3() {
        assert!(!Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 10));
    }
}
