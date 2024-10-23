use b_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn replace_value_in_tree(root: OptNode) -> OptNode {
        let root = root?;
        let mut level_sum = 0;
        let mut queue = vec![(root.clone(), 0)];
        while !queue.is_empty() {
            let mut new_queue = Vec::new();
            let mut new_level_sum = 0;
            for (node, sibling_sum) in queue {
                node.borrow_mut().val = level_sum - sibling_sum;
                let mut subling_sum = 0;
                let mut non_leaf_children = Vec::new();
                if let Some(child) = node.borrow().left.clone() {
                    subling_sum += child.borrow().val;
                    non_leaf_children.push(child);
                }
                if let Some(child) = node.borrow().right.clone() {
                    subling_sum += child.borrow().val;
                    non_leaf_children.push(child);
                }
                new_level_sum += subling_sum;
                non_leaf_children
                    .into_iter()
                    .for_each(|node| new_queue.push((node, subling_sum)));
            }
            queue = new_queue;
            level_sum = new_level_sum;
        }
        Some(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root = TreeNode::from(vec![
            Some(5),
            Some(4),
            Some(9),
            Some(1),
            Some(10),
            None,
            Some(7),
        ]);
        let expected = TreeNode::from(vec![
            Some(0),
            Some(0),
            Some(0),
            Some(7),
            Some(7),
            None,
            Some(11),
        ]);
        assert_eq!(Solution::replace_value_in_tree(root), expected);
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from(vec![Some(3), Some(1), Some(2)]);
        let expected = TreeNode::from(vec![Some(0), Some(0), Some(0)]);
        assert_eq!(Solution::replace_value_in_tree(root), expected);
    }
}
