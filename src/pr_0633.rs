pub struct Solution;

// Fermat Theorem
impl Solution {
    pub const fn judge_square_sum(mut c: i32) -> bool {
        let mut i = 2;
        while i * i <= c {
            let mut count = 0;
            if c % i == 0 {
                while c % i == 0 {
                    count += 1;
                    c /= i;
                }
                if i % 4 == 3 && count % 2 != 0 {
                    return false;
                }
            }
            i += 1;
        }
        c % 4 != 3
    }
}

// Binary Search (fails for big numbers)
// impl Solution {
//     pub fn judge_square_sum(c: i32) -> bool {
//         let mut a = 0;
//         while a * a <= c {
//             let b = c - a * a;
//             if Self::binary_search(0, b, b) {
//                 return true;
//             }
//             a += 1;
//         }
//         false
//     }

//     fn binary_search(s: i32, e: i32, n: i32) -> bool {
//         if s > e {
//             return false;
//         }
//         let mid = s + (e - s) / 2;
//         if mid * mid == n {
//             return true;
//         }
//         if mid * mid > n {
//             return Self::binary_search(s, mid - 1, n);
//         }
//         Self::binary_search(mid + 1, e, n)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::judge_square_sum(5), true);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::judge_square_sum(3), false);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::judge_square_sum(6), false);
    }
}
