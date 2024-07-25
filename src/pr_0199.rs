use b_tree::TreeNode;

pub struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

// use std::{cell::RefCell, rc::Rc};
//
// impl Solution {
//     #[allow(clippy::needless_pass_by_value)]
//     pub fn right_side_view(root: Tree) -> Vec<i32> {
//         let mut right_view = Vec::new();
//         Self::recurse(&root, 0, &mut right_view);
//         right_view
//     }
//
//     fn recurse(node: &Tree, depth: usize, right_view: &mut Vec<i32>) {
//         if let Some(node) = node {
//             let node = node.borrow();
//             match right_view.get_mut(depth) {
//                 Some(val) => *val = node.val,
//                 None => right_view.push(node.val),
//             }
//             if node.left.is_some() {
//                 Self::recurse(&node.left.clone(), depth + 1, right_view);
//             }
//             if node.right.is_some() {
//                 Self::recurse(&node.right.clone(), depth + 1, right_view);
//             }
//         }
//     }
// }

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

impl Solution {
    pub fn right_side_view(root: Tree) -> Vec<i32> {
        let mut right_view = Vec::new();
        if let Some(node) = root {
            let mut queue = VecDeque::from([node]);
            while !queue.is_empty() {
                let bound = queue.len() - 1;
                for i in 0..=bound {
                    let node = queue.pop_front().unwrap();
                    let node = node.borrow();
                    if i == bound {
                        right_view.push(node.val);
                    }
                    if let Some(left) = node.left.clone() {
                        queue.push_back(left);
                    }
                    if let Some(right) = node.right.clone() {
                        queue.push_back(right);
                    }
                }
            }
        }
        right_view
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root = TreeNode::from(vec![
            Some(1),
            Some(2),
            Some(3),
            None,
            Some(5),
            None,
            Some(4),
        ]);
        assert_eq!(vec![1, 3, 4], Solution::right_side_view(root));
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from(vec![Some(1), None, Some(3)]);
        assert_eq!(vec![1, 3], Solution::right_side_view(root));
    }

    #[test]
    fn case_3() {
        let root = TreeNode::from(vec![]);
        assert_eq!(Vec::<i32>::new(), Solution::right_side_view(root));
    }

    #[test]
    fn case_4() {
        let root = TreeNode::from(vec![Some(1), Some(2), Some(3), None, Some(5), Some(4)]);
        assert_eq!(vec![1, 3, 4], Solution::right_side_view(root));
    }

    #[test]
    fn case_5() {
        let root = TreeNode::from(vec![Some(1), Some(2), Some(3), Some(4)]);
        assert_eq!(vec![1, 3, 4], Solution::right_side_view(root));
    }
}
