use b_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        root.map_or(false, |node| {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() && node.val == target_sum {
                true
            } else {
                Self::has_path_sum(node.left.clone(), target_sum - node.val)
                    || Self::has_path_sum(node.right.clone(), target_sum - node.val)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let tree = TreeNode::from(vec![
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            None,
            Some(1),
        ]);
        assert!(Solution::has_path_sum(tree, 22));
    }

    #[test]
    fn case_2() {
        let tree = TreeNode::from(vec![Some(1), Some(2), Some(3)]);
        assert!(!Solution::has_path_sum(tree, 5));
    }

    #[test]
    fn case_3() {
        let tree = TreeNode::from(vec![]);
        assert!(!Solution::has_path_sum(tree, 0));
    }
}
