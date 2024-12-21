use b_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn reverse_odd_levels(root: OptNode) -> OptNode {
        Self::dfs(&root, &None, 0);
        root
    }

    fn dfs(left: &OptNode, right: &OptNode, level: i32) {
        let rev = level & 0x01 == 1;
        match (left.clone(), right.clone()) {
            (Some(lnode), None) => {
                Self::dfs(&lnode.borrow().left, &lnode.borrow().right, level + 1);
            }
            (Some(lnode), Some(rnode)) => {
                if rev {
                    let (lval, rval) = (lnode.borrow().val, rnode.borrow().val);
                    (lnode.borrow_mut().val, rnode.borrow_mut().val) = (rval, lval);
                }
                Self::dfs(&lnode.borrow().left, &rnode.borrow().right, level + 1);
                Self::dfs(&lnode.borrow().right, &rnode.borrow().left, level + 1);
            }
            (None, Some(rnode)) => {
                Self::dfs(&rnode.borrow().left, &rnode.borrow().right, level + 1);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root = TreeNode::from(vec![
            Some(2),
            Some(3),
            Some(5),
            Some(8),
            Some(13),
            Some(21),
            Some(34),
        ]);
        let expected = TreeNode::from(vec![
            Some(2),
            Some(5),
            Some(3),
            Some(8),
            Some(13),
            Some(21),
            Some(34),
        ]);
        assert_eq!(Solution::reverse_odd_levels(root), expected);
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from(vec![Some(7), Some(13), Some(11)]);
        let expected = TreeNode::from(vec![Some(7), Some(11), Some(13)]);
        assert_eq!(Solution::reverse_odd_levels(root), expected);
    }
}
