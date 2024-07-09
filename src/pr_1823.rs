pub struct Solution;

// impl Solution {
//     pub fn find_the_winner(n: i32, k: i32) -> i32 {
//         Self::winner_helper(n, k) + 1
//     }
//
//     fn winner_helper(n: i32, k: i32) -> i32 {
//         if n == 1 {
//             return 0;
//         }
//         (Self::winner_helper(n - 1, k) + k) % n
//     }
// }

impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut ans = 0;
        for i in 2..=n {
            ans = (ans + k) % i;
        }
        ans + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::find_the_winner(5, 2), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::find_the_winner(6, 5), 1);
    }
}
