use b_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn is_valid_bst(root: Tree) -> bool {
        Self::is_valid(&root, i64::MIN, i64::MAX)
    }

    fn is_valid(node: &Tree, left_boundry: i64, right_boundry: i64) -> bool {
        node.as_ref().map_or(true, |node| {
            let node = node.borrow();
            let val = i64::from(node.val);
            left_boundry < val
                && val < right_boundry
                && Self::is_valid(&node.left, left_boundry, val)
                && Self::is_valid(&node.right, val, right_boundry)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root = TreeNode::from(vec![Some(2), Some(1), Some(3)]);
        assert!(Solution::is_valid_bst(root));
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from(vec![
            Some(5),
            Some(1),
            Some(4),
            None,
            None,
            Some(3),
            Some(6),
        ]);
        assert!(!Solution::is_valid_bst(root));
    }

    #[test]
    fn case_3() {
        let root = TreeNode::from(vec![
            Some(5),
            Some(1),
            Some(6),
            None,
            None,
            Some(3),
            Some(7),
        ]);
        assert!(!Solution::is_valid_bst(root));
    }

    #[test]
    fn case_4() {
        let root = TreeNode::from(vec![Some(2_147_483_647)]);
        assert!(Solution::is_valid_bst(root));
    }

    #[test]
    fn case_5() {
        let root = TreeNode::from(vec![Some(2), Some(2), Some(2)]);
        assert!(!Solution::is_valid_bst(root));
    }
}
