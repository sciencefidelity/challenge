use b_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let tree = TreeNode::from(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert!(Solution::is_balanced(tree));
    }

    #[test]
    fn case_2() {
        let tree = TreeNode::from(vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(3),
            None,
            None,
            Some(4),
            Some(4),
        ]);
        assert!(!Solution::is_balanced(tree));
    }

    #[test]
    fn case_3() {
        let tree = TreeNode::from(vec![]);
        assert!(Solution::is_balanced(tree));
    }
}
