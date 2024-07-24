use b_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn sorted_array_to_bst(nums: impl AsRef<[i32]>) -> Tree {
        let nums = nums.as_ref();
        if nums.is_empty() {
            None
        } else {
            let (left, rest) = nums.split_at(nums.len() / 2);
            let (&val, right) = rest.split_first().unwrap();
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: Self::sorted_array_to_bst(left),
                right: Self::sorted_array_to_bst(right),
            })))
        }
    }
}

// impl Solution {
//     pub fn sorted_array_to_bst(mut nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
//         let right = nums.split_off((nums.len() + 1) / 2);
//         nums.pop().map(|val| {
//             Rc::new(RefCell::new(TreeNode {
//                 val,
//                 left: Self::sorted_array_to_bst(nums),
//                 right: Self::sorted_array_to_bst(right),
//             }))
//         })
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let expected = TreeNode::from(vec![Some(0), Some(-3), Some(9), Some(-10), None, Some(5)]);
        assert_eq!(
            Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]),
            expected
        );
    }

    #[test]
    fn case_2() {
        let expected = TreeNode::from(vec![Some(3), Some(1)]);
        assert_eq!(Solution::sorted_array_to_bst(vec![1, 3]), expected);
    }
}
