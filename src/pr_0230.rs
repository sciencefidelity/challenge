use b_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn kth_smallest(root: Tree, k: i32) -> i32 {
        Self::bfs(root, k)
    }

    pub fn bfs(mut maybe_node: Tree, mut k: i32) -> i32 {
        let mut stack = Vec::new();
        loop {
            while let Some(node) = maybe_node {
                maybe_node = node.borrow_mut().left.take();
                stack.push(node);
            }
            if let Some(node) = stack.pop() {
                k -= 1;
                if k == 0 {
                    return node.borrow().val;
                }
                maybe_node = node.borrow_mut().right.take();
            } else {
                unreachable!();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root = TreeNode::from(vec![Some(3), Some(1), Some(4), None, Some(2)]);
        assert_eq!(1, Solution::kth_smallest(root, 1));
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from(vec![
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            None,
            Some(1),
        ]);
        assert_eq!(3, Solution::kth_smallest(root, 3));
    }
}
