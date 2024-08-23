use b_tree::TreeNode;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

pub struct Solution;

impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (mut max, mut max_level, mut level) = (i32::MIN, 1, 1);
        if let Some(node_) = root {
            let mut queue = VecDeque::new();
            queue.push_back(node_);
            while !queue.is_empty() {
                let mut sum = 0;
                for _ in (0..queue.len()).rev() {
                    let node = queue.pop_front().unwrap();
                    let node = node.borrow();
                    sum += node.val;
                    if let Some(left) = node.left.clone() {
                        queue.push_back(left);
                    }
                    if let Some(right) = node.right.clone() {
                        queue.push_back(right);
                    }
                }
                if max < sum {
                    (max, max_level) = (sum, level);
                }
                level += 1;
            }
        }
        max_level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root = TreeNode::from(vec![
            Some(1),
            Some(7),
            Some(0),
            Some(7),
            Some(-8),
            None,
            None,
        ]);
        assert_eq!(Solution::max_level_sum(root), 2);
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from(vec![
            Some(989),
            None,
            Some(10250),
            Some(98693),
            Some(-89388),
            None,
            None,
            None,
            Some(-32127),
        ]);
        assert_eq!(Solution::max_level_sum(root), 2);
    }
}
