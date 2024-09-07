use b_tree::TreeNode;
use linked::ListNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

type OptList = Option<Box<ListNode>>;
type OptTree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn is_sub_path(head: OptList, root: OptTree) -> bool {
        if root.is_none() {
            return false;
        }
        Self::check(&root, &head)
    }

    fn check(node: &OptTree, head: &OptList) -> bool {
        if Self::dfs(node, head) {
            return true;
        }
        node.as_ref().map_or(false, |node| {
            let node = node.borrow();
            Self::check(&node.left, head) || Self::check(&node.right, head)
        })
    }

    fn dfs(node: &OptTree, head: &OptList) -> bool {
        if head.is_none() {
            return true;
        }
        if let Some(node) = node {
            let (node, head) = (node.borrow(), head.as_ref().unwrap());
            if node.val == head.val {
                return Self::dfs(&node.left, &head.next) || Self::dfs(&node.right, &head.next);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let head = ListNode::from(&[4, 2, 8]);
        let root = TreeNode::from(vec![
            Some(1),
            Some(4),
            Some(4),
            None,
            Some(2),
            Some(2),
            None,
            Some(1),
            None,
            Some(6),
            Some(8),
            None,
            None,
            None,
            None,
            Some(1),
            Some(3),
        ]);
        assert!(Solution::is_sub_path(head, root));
    }

    #[test]
    fn case_2() {
        let head = ListNode::from(&[1, 4, 2, 6]);
        let root = TreeNode::from(vec![
            Some(1),
            Some(4),
            Some(4),
            None,
            Some(2),
            Some(2),
            None,
            Some(1),
            None,
            Some(6),
            Some(8),
            None,
            None,
            None,
            None,
            Some(1),
            Some(3),
        ]);
        assert!(Solution::is_sub_path(head, root));
    }

    #[test]
    fn case_3() {
        let head = ListNode::from(&[1, 4, 2, 6, 8]);
        let root = TreeNode::from(vec![
            Some(1),
            Some(4),
            Some(4),
            None,
            Some(2),
            Some(2),
            None,
            Some(1),
            None,
            Some(6),
            Some(8),
            None,
            None,
            None,
            None,
            Some(1),
            Some(3),
        ]);
        assert!(!Solution::is_sub_path(head, root));
    }
}
