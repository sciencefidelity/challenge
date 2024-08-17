use b_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn good_nodes(root: OptNode) -> i32 {
        Self::recurse(root, i32::MIN)
    }

    fn recurse(maybe_node: OptNode, mut path_max: i32) -> i32 {
        maybe_node.map_or(0, |node| {
            let node = node.borrow();
            path_max = path_max.max(node.val);
            i32::from(node.val >= path_max)
                + Self::recurse(node.left.clone(), path_max)
                + Self::recurse(node.right.clone(), path_max)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root = TreeNode::from(vec![
            Some(3),
            Some(1),
            Some(4),
            Some(3),
            None,
            Some(1),
            Some(5),
        ]);
        assert_eq!(4, Solution::good_nodes(root));
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from(vec![Some(3), Some(3), None, Some(4), Some(2)]);
        assert_eq!(3, Solution::good_nodes(root));
    }
}
