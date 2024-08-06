use b_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn postorder_traversal(root: OptNode) -> Vec<i32> {
        let mut output = Vec::new();
        Self::recurse(&root, &mut output);
        output
    }

    fn recurse(maybe_node: &OptNode, output: &mut Vec<i32>) {
        if let Some(node) = maybe_node {
            let node = node.borrow();
            Self::recurse(&node.left, output);
            Self::recurse(&node.right, output);
            output.push(node.val);
        }
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
