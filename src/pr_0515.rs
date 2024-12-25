use b_tree::TreeNode;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

type OptNode = Option<Rc<RefCell<TreeNode>>>;

pub struct Solution;

impl Solution {
    pub fn largest_values(root: OptNode) -> Vec<i32> {
        let mut largest_values = Vec::new();
        if let Some(root) = root {
            let mut queue = VecDeque::new();
            queue.push_back(root);
            while !queue.is_empty() {
                let mut max_level = i32::MIN;
                for _ in 0..queue.len() {
                    let node_ref = queue.pop_front().unwrap();
                    let node = node_ref.borrow();
                    max_level = max_level.max(node.val);
                    if let Some(left) = node.left.clone() {
                        queue.push_back(left);
                    }
                    if let Some(right) = node.right.clone() {
                        queue.push_back(right);
                    }
                }
                largest_values.push(max_level);
            }
        }
        largest_values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root = TreeNode::from(vec![
            Some(1),
            Some(3),
            Some(2),
            Some(5),
            Some(3),
            None,
            Some(9),
        ]);
        assert_eq!(Solution::largest_values(root), vec![1, 3, 9]);
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from(vec![Some(1), Some(3), Some(2)]);
        assert_eq!(Solution::largest_values(root), vec![1, 3]);
    }
}
