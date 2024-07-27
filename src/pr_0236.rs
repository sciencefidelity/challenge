use b_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn lowest_common_ancestor(root: Tree, p: Tree, q: Tree) -> Tree {
        Self::count_matches(&root, p.as_ref()?.borrow().val, q.as_ref()?.borrow().val).0
    }

    fn count_matches(maybe_node: &Tree, p: i32, q: i32) -> (Tree, i32) {
        if let Some(node) = maybe_node.as_ref() {
            let node_ref = node.borrow();
            let left = Self::count_matches(&node_ref.left, p, q);
            if left.0.is_some() {
                return left;
            }
            let right = Self::count_matches(&node_ref.right, p, q);
            if right.0.is_some() {
                return right;
            }
            let count = left.1 + right.1 + i32::from(node_ref.val == p || node_ref.val == q);
            let found = if count == 2 { Some(node.clone()) } else { None };
            (found, count)
        } else {
            (None, 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root = TreeNode::from(vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let p = TreeNode::from(vec![
            Some(5),
            Some(6),
            Some(2),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let q = TreeNode::from(vec![Some(1), Some(0), Some(8)]);
        let expected = TreeNode::from(vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        assert_eq!(expected, Solution::lowest_common_ancestor(root, p, q));
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from(vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let p = TreeNode::from(vec![
            Some(5),
            Some(6),
            Some(2),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let q = TreeNode::from(vec![Some(4)]);
        let expected = TreeNode::from(vec![
            Some(5),
            Some(6),
            Some(2),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        assert_eq!(expected, Solution::lowest_common_ancestor(root, p, q));
    }
}
