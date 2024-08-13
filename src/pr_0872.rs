use b_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn leaf_similar(root1: Tree, root2: Tree) -> bool {
        let (mut leaves1, mut leaves2) = (Vec::new(), Vec::new());
        Self::find_leaves(root1, &mut leaves1);
        Self::find_leaves(root2, &mut leaves2);
        leaves1 == leaves2
    }

    fn find_leaves(node: Tree, leaves: &mut Vec<i32>) {
        if let Some(node) = node {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                return leaves.push(node.val);
            }
            Self::find_leaves(node.left.clone(), leaves);
            Self::find_leaves(node.right.clone(), leaves);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root1 = TreeNode::from(vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(9),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let root2 = TreeNode::from(vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(7),
            Some(4),
            Some(2),
            None,
            None,
            None,
            None,
            None,
            None,
            Some(9),
            Some(8),
        ]);
        assert!(Solution::leaf_similar(root1, root2));
    }

    #[test]
    fn case_2() {
        let root1 = TreeNode::from(vec![Some(1), Some(2), Some(3)]);
        let root2 = TreeNode::from(vec![Some(1), Some(3), Some(2)]);
        assert!(!Solution::leaf_similar(root1, root2));
    }
}
