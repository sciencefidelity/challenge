pub struct Solution;

// impl Solution {
//     pub fn lemonade_change(bills: Vec<i32>) -> bool {
//         bills
//             .into_iter()
//             .skip(1)
//             .try_fold((1, 0), |(a, b), x| match x {
//                 5 => Some((a + 1, b)),
//                 10 if a > 0 => Some((a - 1, b + 1)),
//                 20 if a > 0 && b > 0 => Some((a - 1, b - 1)),
//                 20 if a > 2 && b == 0 => Some((a - 3, b)),
//                 _ => None,
//             })
//             .is_some()
//     }
// }

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        Self::helper(&bills, bills.len())
    }

    const fn helper(bills: &[i32], n: usize) -> bool {
        if bills[0] != 5 || (n > 1 && bills[1] == 20) {
            return false;
        }
        let mut fives = 1;
        let mut tens = 0;
        let mut i = 1;
        while i < n {
            match bills[i] {
                5 => fives += 1,
                10 => {
                    if fives > 0 {
                        fives -= 1;
                        tens += 1;
                    } else {
                        return false;
                    }
                }
                20 => {
                    if fives > 0 && tens > 0 {
                        fives -= 1;
                        tens -= 1;
                    } else if fives > 2 {
                        fives -= 3;
                    } else {
                        return false;
                    }
                }
                _ => {}
            }
            i += 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::lemonade_change(vec![5, 5, 5, 10, 20]));
    }

    #[test]
    fn case_2() {
        assert!(!Solution::lemonade_change(vec![5, 5, 10, 10, 20]));
    }

    #[test]
    fn case_3() {
        assert!(Solution::lemonade_change(vec![5]));
    }
}
