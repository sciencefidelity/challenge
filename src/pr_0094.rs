use b_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn inorder_traversal(root: OptNode) -> Vec<i32> {
        let mut output = Vec::new();
        Self::recurse(&root, &mut output);
        output
    }

    fn recurse(maybe_node: &OptNode, order: &mut Vec<i32>) {
        if let Some(node) = maybe_node {
            let node = node.borrow();
            Self::recurse(&node.left, order);
            order.push(node.val);
            Self::recurse(&node.right, order);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root = TreeNode::from(vec![Some(1), None, Some(2), Some(3)]);
        assert_eq!(vec![1, 3, 2], Solution::inorder_traversal(root));
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from(vec![Some(1)]);
        assert_eq!(vec![1], Solution::inorder_traversal(root));
    }
}
