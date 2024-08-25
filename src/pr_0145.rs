use b_tree::TreeNode;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

pub struct Solution;

type OptNode = Option<Rc<RefCell<TreeNode>>>;

// impl Solution {
//     #[allow(clippy::needless_pass_by_value)]
//     pub fn postorder_traversal(root: OptNode) -> Vec<i32> {
//         let mut result = Vec::new();
//         Self::postorder(&root, &mut output);
//         result
//     }
//
//     fn postorder(maybe_node: &OptNode, output: &mut Vec<i32>) {
//         if let Some(node) = maybe_node {
//             Self::postorder(&node.borrow().left, output);
//             Self::postorder(&node.borrow().right, output);
//             result.push(node.borrow().val);
//         }
//     }
// }

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn postorder_traversal(root: OptNode) -> Vec<i32> {
        let mut stack = Vec::from_iter(root);
        let mut result = VecDeque::with_capacity(100);
        while let Some(node) = stack.pop() {
            if let Some(left) = node.borrow().left.clone() {
                stack.push(left);
            }
            if let Some(right) = node.borrow().right.clone() {
                stack.push(right);
            }
            result.push_front(node.borrow().val);
        }
        result.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            vec![3, 2, 1],
            Solution::postorder_traversal(TreeNode::from(vec![Some(1), None, Some(2), Some(3)]))
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::postorder_traversal(TreeNode::from(vec![]))
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            vec![1],
            Solution::postorder_traversal(TreeNode::from(vec![Some(1)]))
        );
    }

    #[test]
    fn case_4() {
        let root = TreeNode::from(vec![
            Some(1),
            Some(2),
            Some(7),
            Some(3),
            Some(4),
            None,
            Some(8),
            None,
            None,
            Some(5),
            Some(6),
            None,
            Some(9),
        ]);
        assert_eq!(
            vec![3, 5, 6, 4, 2, 9, 8, 7, 1],
            Solution::postorder_traversal(root)
        );
    }
}
