use b_tree::TreeNode;
use std::{cell::RefCell, cmp::max, rc::Rc};

pub struct Solution;

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn longest_zig_zag(root: OptNode) -> i32 {
        let mut result = 0;
        if let Some(node) = root {
            Self::path(&node.borrow().left, &mut result, 0, false);
            Self::path(&node.borrow().right, &mut result, 0, true);
        }
        result
    }

    fn path(maybe_node: &OptNode, result: &mut i32, depth: i32, direction: bool) {
        *result = max(*result, depth);
        if let Some(node) = maybe_node {
            Self::path(
                &node.borrow().left,
                result,
                if direction { depth + 1 } else { 0 },
                false,
            );
            Self::path(
                &node.borrow().right,
                result,
                if direction { 0 } else { depth + 1 },
                true,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root = TreeNode::from(vec![
            Some(1),
            None,
            Some(1),
            Some(1),
            Some(1),
            None,
            None,
            Some(1),
            Some(1),
            None,
            Some(1),
            None,
            None,
            None,
            Some(1),
        ]);
        assert_eq!(Solution::longest_zig_zag(root), 3);
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from(vec![
            Some(1),
            Some(1),
            Some(1),
            None,
            Some(1),
            None,
            None,
            Some(1),
            Some(1),
            None,
            Some(1),
        ]);
        assert_eq!(Solution::longest_zig_zag(root), 4);
    }

    #[test]
    fn case_3() {
        let root = TreeNode::from(vec![Some(1)]);
        assert_eq!(Solution::longest_zig_zag(root), 0);
    }
}
