use b_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn sum_numbers(root: Tree) -> i32 {
        Self::recurse(root, 0)
    }

    fn recurse(node: Tree, to_leaf: i32) -> i32 {
        node.map_or(0, |node| {
            let node = node.borrow();
            let sum = to_leaf * 10 + node.val;
            if node.left.is_none() && node.right.is_none() {
                sum
            } else {
                Self::recurse(node.left.clone(), sum) + Self::recurse(node.right.clone(), sum)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root = TreeNode::from(vec![Some(1), Some(2), Some(3)]);
        assert_eq!(25, Solution::sum_numbers(root));
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from(vec![Some(4), Some(9), Some(0), Some(5), Some(1)]);
        assert_eq!(1026, Solution::sum_numbers(root));
    }
}
