use b_tree::TreeNode;
use std::{cell::RefCell, cmp::min, rc::Rc};

pub struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    #[allow(clippy::assigning_clones)]
    pub fn get_minimum_difference(root: Tree) -> i32 {
        let mut min_diff = i32::MAX;
        let mut curr = root;
        let mut stack = Vec::new();
        let mut pre = -1;
        while !stack.is_empty() || curr.is_some() {
            while let Some(node) = curr {
                curr = node.borrow().left.clone();
                stack.push(node);
            }
            if let Some(node) = stack.pop() {
                let node = node.borrow();
                if pre >= 0 {
                    min_diff = min(min_diff, node.val - pre);
                }
                pre = node.val;
                curr.clone_from(&node.right);
            }
        }
        min_diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root = TreeNode::from(vec![Some(4), Some(2), Some(6), Some(1), Some(3)]);
        assert_eq!(1, Solution::get_minimum_difference(root));
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from(vec![
            Some(1),
            Some(0),
            Some(48),
            None,
            None,
            Some(12),
            Some(49),
        ]);
        assert_eq!(1, Solution::get_minimum_difference(root));
    }

    #[test]
    fn case_3() {
        let root = TreeNode::from(vec![
            Some(236),
            Some(104),
            Some(701),
            None,
            Some(227),
            None,
            Some(911),
        ]);
        assert_eq!(9, Solution::get_minimum_difference(root));
    }
}
