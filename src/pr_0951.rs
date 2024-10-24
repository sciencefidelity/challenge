use b_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn flip_equiv(root1: OptNode, root2: OptNode) -> bool {
        if root1.is_none() && root2.is_none() {
            return true;
        }
        if root1.is_none() || root2.is_none() {
            return false;
        }
        if let (Some(root1), Some(root2)) = (root1, root2) {
            let (root1, root2) = (root1.borrow(), root2.borrow());
            if root1.val != root2.val {
                return false;
            }

            let no_swap = Self::flip_equiv(root1.left.clone(), root2.left.clone())
                && Self::flip_equiv(root1.right.clone(), root2.right.clone());
            let swap = Self::flip_equiv(root1.left.clone(), root2.right.clone())
                && Self::flip_equiv(root1.right.clone(), root2.left.clone());

            return no_swap || swap;
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root1 = TreeNode::from(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            None,
            None,
            None,
            Some(7),
            Some(8),
        ]);
        let root2 = TreeNode::from(vec![
            Some(1),
            Some(3),
            Some(2),
            None,
            Some(6),
            Some(4),
            Some(5),
            None,
            None,
            None,
            None,
            Some(8),
            Some(7),
        ]);
        assert!(Solution::flip_equiv(root1, root2));
    }

    #[test]
    fn case_2() {
        let root1 = TreeNode::from(vec![]);
        let root2 = TreeNode::from(vec![]);
        assert!(Solution::flip_equiv(root1, root2));
    }

    #[test]
    fn case_3() {
        let root1 = TreeNode::from(vec![]);
        let root2 = TreeNode::from(vec![Some(1)]);
        assert!(!Solution::flip_equiv(root1, root2));
    }
}
