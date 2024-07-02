pub struct Solution;

use b_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_balanced(root: Tree) -> bool {
        Self::dfs(&root).is_some()
    }

    fn dfs(root: &Tree) -> Option<i32> {
        match &root {
            None => Some(0),
            Some(node) => {
                let left = Self::dfs(&node.borrow().left)?;
                let right = Self::dfs(&node.borrow().right)?;
                if (left - right).abs() > 1 {
                    return None;
                }
                Some(left.max(right) + 1)
            }
        }
    }
}

// TODO: write tests
