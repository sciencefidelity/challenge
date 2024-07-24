use b_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn flatten(root: &mut Tree) {
        *root = Self::recurse(root.take(), None);
    }

    fn recurse(node: Tree, after: Tree) -> Tree {
        match &node {
            None => return after,
            Some(n) => {
                let mut node_ref = n.borrow_mut();
                let right = Self::recurse(node_ref.right.take(), after);
                node_ref.right = Self::recurse(node_ref.left.take(), right);
            }
        }
        node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut root = TreeNode::from(vec![
            Some(1),
            None,
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
            None,
            Some(6),
        ]);
        let expected = TreeNode::from(vec![
            Some(1),
            None,
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
            None,
            Some(6),
        ]);
        Solution::flatten(&mut root);
        assert_eq!(expected, root);
    }

    #[test]
    fn case_2() {
        let mut root = TreeNode::from(vec![]);
        let expected = TreeNode::from(vec![]);
        Solution::flatten(&mut root);
        assert_eq!(expected, root);
    }

    #[test]
    fn case_3() {
        let mut root = TreeNode::from(vec![Some(0)]);
        let expected = TreeNode::from(vec![Some(0)]);
        Solution::flatten(&mut root);
        assert_eq!(expected, root);
    }
}
