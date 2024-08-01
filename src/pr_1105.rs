use std::cmp::{max, min};

pub struct Solution;

// impl Solution {
//     #[allow(clippy::needless_pass_by_value)]
//     pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
//         let mut memo = vec![vec![0; (shelf_width + 1).try_into().unwrap()]; books.len()];
//
//         Self::dp_helper(&books, shelf_width, &mut memo, 0, shelf_width, 0)
//     }
//
//     #[allow(clippy::branches_sharing_code)]
//     fn dp_helper(
//         books: &[Vec<i32>],
//         shelf_width: i32,
//         memo: &mut [Vec<i32>],
//         i: usize,
//         remaining_shelf_width: i32,
//         max_height: i32,
//     ) -> i32 {
//         let remaining_shelf_width = usize::try_from(remaining_shelf_width).unwrap();
//         let current_book = &books[i];
//         let max_height_updated = max(max_height, current_book[1]);
//         if i == books.len() - 1 && i32::try_from(remaining_shelf_width).unwrap() >= current_book[0]
//         {
//             return max_height_updated;
//         }
//         if memo[i][remaining_shelf_width] != 0 {
//             memo[i][remaining_shelf_width]
//         } else {
//             let optional_height = max_height
//                 + Self::dp_helper(
//                     books,
//                     shelf_width,
//                     memo,
//                     i + 1,
//                     shelf_width - current_book[0],
//                     current_book[1],
//                 );
//             if i32::try_from(remaining_shelf_width).unwrap() >= current_book[0] {
//                 let option2_height = Self::dp_helper(
//                     books,
//                     shelf_width,
//                     memo,
//                     i + 1,
//                     i32::try_from(remaining_shelf_width).unwrap() - current_book[0],
//                     max_height_updated,
//                 );
//                 memo[i][remaining_shelf_width] = min(optional_height, option2_height);
//                 return memo[i][remaining_shelf_width];
//             }
//             memo[i][remaining_shelf_width] = optional_height;
//             memo[i][remaining_shelf_width]
//         }
//     }
// }

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let mut dp = vec![0; books.len() + 1];
        dp[0] = 0;
        dp[1] = books[0][1];
        for i in 1..=books.len() {
            let mut remaining_shelf_width = shelf_width - books[i - 1][0];
            let mut max_height = books[i - 1][1];
            dp[i] = dp[i - 1] + max_height;
            let mut j = i - 1;
            while j > 0 && remaining_shelf_width - books[j - 1][0] >= 0 {
                remaining_shelf_width -= books[j - 1][0];
                max_height = max(max_height, books[j - 1][1]);
                dp[i] = min(dp[i], dp[j - 1] + max_height);
                j -= 1;
            }
        }

        dp[books.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let books = vec![
            vec![1, 1],
            vec![2, 3],
            vec![2, 3],
            vec![1, 1],
            vec![1, 1],
            vec![1, 1],
            vec![1, 2],
        ];
        assert_eq!(6, Solution::min_height_shelves(books, 4));
    }

    #[test]
    fn case_2() {
        let books = vec![vec![1, 3], vec![2, 4], vec![3, 2]];
        assert_eq!(4, Solution::min_height_shelves(books, 6));
    }
}
