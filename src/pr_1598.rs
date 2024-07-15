pub struct Solution;

// impl Solution {
//     pub fn min_operations(logs: Vec<String>) -> i32 {
//         let mut operations = 0;
//         for operation in logs {
//             match operation.as_str() {
//                 "../" if operations > 0 => operations -= 1,
//                 "./" | "../" => continue,
//                 _ => operations += 1,
//             }
//         }
//         operations
//     }
// }

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        logs.into_iter()
            .fold(0, |depth, operation| match operation.as_str() {
                "../" if depth > 0 => depth - 1,
                "./" | "../" => depth,
                _ => depth + 1,
            })
    }
}

// impl Solution {
//     pub fn min_operations(logs: Vec<String>) -> i32 {
//         let mut folder_stack = Vec::with_capacity(logs.len());
//         for operation in logs {
//             match operation.as_str() {
//                 "../" if !folder_stack.is_empty() => {
//                     folder_stack.pop();
//                 }
//                 "./" | "../" => continue,
//                 _ => {
//                     folder_stack.push(operation);
//                 }
//             }
//         }
//         folder_stack.len() as i32
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::min_operations(vec![
                "d1/".to_owned(),
                "d2/".to_owned(),
                "../".to_owned(),
                "d21/".to_owned(),
                "./".to_owned()
            ]),
            2
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::min_operations(vec![
                "d1/".to_owned(),
                "d2/".to_owned(),
                "./".to_owned(),
                "d3/".to_owned(),
                "../".to_owned(),
                "d31/".to_owned()
            ]),
            3
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::min_operations(vec![
                "d1/".to_owned(),
                "../".to_owned(),
                "../".to_owned(),
                "../".to_owned()
            ]),
            0
        );
    }
}
