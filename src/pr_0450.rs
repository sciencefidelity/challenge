use b_tree::TreeNode;
use std::{cell::RefCell, cmp::Ordering, rc::Rc};

pub struct Solution;

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn delete_node(root: OptNode, key: i32) -> OptNode {
        Self::dfs(&root, key)
    }

    fn dfs(node: &OptNode, key: i32) -> OptNode {
        if let Some(n) = node {
            let val = n.borrow().val;
            match val.cmp(&key) {
                Ordering::Greater => {
                    let l = Self::dfs(&n.borrow().left, key);
                    n.borrow_mut().left = l;
                }
                Ordering::Less => {
                    let r = Self::dfs(&n.borrow().right, key);
                    n.borrow_mut().right = r;
                }
                Ordering::Equal => {
                    if n.borrow().left.is_none() {
                        return n.borrow().right.clone();
                    }
                    if n.borrow().right.is_none() {
                        return n.borrow().left.clone();
                    }
                    let next = Self::search(&n.borrow().right);
                    if let Some(val) = next {
                        let r = Self::dfs(&n.borrow().right, val);
                        n.borrow_mut().val = val;
                        n.borrow_mut().right = r;
                    }
                }
            }
        }
        node.clone()
    }

    fn search(node: &OptNode) -> Option<i32> {
        node.as_ref().and_then(|n| {
            if n.borrow().left.is_some() {
                Self::search(&n.borrow().left)
            } else {
                Some(n.borrow().val)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root = TreeNode::from(vec![
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            Some(7),
        ]);
        let expected = TreeNode::from(vec![
            Some(5),
            Some(4),
            Some(6),
            Some(2),
            None,
            None,
            Some(7),
        ]);
        assert_eq!(Solution::delete_node(root, 3), expected);
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
            Some(7),
        ]);
        let expected = TreeNode::from(vec![
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            Some(7),
        ]);
        assert_eq!(Solution::delete_node(root, 0), expected);
    }

    #[test]
    fn case_3() {
        let root = TreeNode::from(vec![]);
        let expected = TreeNode::from(vec![]);
        assert_eq!(Solution::delete_node(root, 0), expected);
    }
}
