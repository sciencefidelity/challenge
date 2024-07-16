pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use b_tree::TreeNode;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn invert_tree(root: Tree) -> Tree {
        if let Some(root) = &root {
            if let Ok(mut root) = root.try_borrow_mut() {
                let right = root.right.clone();
                root.right = Self::invert_tree(root.left.clone());
                root.left = Self::invert_tree(right);
            }
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let tree = TreeNode::from(vec![
            Some(4),
            Some(2),
            Some(7),
            Some(1),
            Some(3),
            Some(6),
            Some(9),
        ]);
        let solution = TreeNode::from(vec![
            Some(4),
            Some(7),
            Some(2),
            Some(9),
            Some(6),
            Some(3),
            Some(1),
        ]);
        assert_eq!(Solution::invert_tree(tree), solution);
    }

    #[test]
    fn case_2() {
        let tree = TreeNode::from(vec![Some(2), Some(1), Some(3)]);
        let solution = TreeNode::from(vec![Some(2), Some(3), Some(1)]);
        assert_eq!(Solution::invert_tree(tree), solution);
    }

    #[test]
    fn case_3() {
        let root = None;
        assert_eq!(Solution::invert_tree(root), None);
    }
}
