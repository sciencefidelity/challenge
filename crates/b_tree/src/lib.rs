#![allow(clippy::use_self, clippy::must_use_candidate)]

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    /// Create a new tree node.
    #[inline]
    pub const fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    /// Create binary tree from a vector.
    ///
    /// # Panics
    ///
    /// Will panic if the input vector is malformed.
    pub fn from(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if vec.is_empty() {
            return None;
        }
        let mut nodes = VecDeque::with_capacity(vec.len());
        let mut iter = vec.into_iter();
        let root = if let Some(val) = iter
            .next()
            .expect("checked if vec is empty so cannot be None here")
        {
            Rc::new(RefCell::new(Self::new(val)))
        } else {
            return None;
        };
        nodes.push_back(root.clone());

        while let Some(val) = iter.next() {
            if let Some(curr_node) = nodes.pop_front() {
                if let Some(left_val) = val {
                    curr_node.borrow_mut().left = Some(Rc::new(RefCell::new(Self::new(left_val))));
                    nodes.push_back(
                        curr_node
                            .borrow_mut()
                            .left
                            .clone()
                            .expect("node created as Some on previous line"),
                    );
                }
                if let Some(Some(right_val)) = iter.next() {
                    curr_node.borrow_mut().right =
                        Some(Rc::new(RefCell::new(Self::new(right_val))));
                    nodes.push_back(
                        curr_node
                            .borrow_mut()
                            .right
                            .clone()
                            .expect("node created as Some on previous line"),
                    );
                }
            }
        }

        Some(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let _tree = TreeNode::from(vec![Some(1), Some(2), None, None, Some(3), Some(4)]);
        // println!("{tree:#?}");
        // assert!(false)
    }

    #[test]
    fn test_2() {
        let _tree = TreeNode::from(vec![]);
        // println!("{tree:#?}");
        // assert!(false)
    }

    #[test]
    fn test_3() {
        let _tree = TreeNode::from(vec![None]);
        // println!("{tree:#?}");
        // assert!(false)
    }
}
