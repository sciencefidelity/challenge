use b_tree::TreeNode;
use std::{cell::RefCell, cmp::min, rc::Rc};

pub struct Solution;

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn min_depth(root: OptNode) -> i32 {
        Self::dfs(&root)
    }

    fn dfs(maybe_node: &OptNode) -> i32 {
        if let Some(node) = maybe_node {
            let node = node.borrow();
            let left = Self::dfs(&node.left);
            let right = Self::dfs(&node.right);
            if left == 0 || right == 0 {
                return left + right + 1;
            }
            return min(left, right) + 1;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root = TreeNode::from(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(2, Solution::min_depth(root));
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from(vec![
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
            None,
            Some(7),
        ]);
        assert_eq!(5, Solution::min_depth(root));
    }
}
