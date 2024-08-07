use b_tree::TreeNode;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

pub struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut level_order: Vec<Vec<i32>> = Vec::new();
        if let Some(node) = root {
            let mut queue = VecDeque::from([node]);
            let mut i = 0;
            while !queue.is_empty() {
                let bound = queue.len() - 1;
                level_order.push(vec![]);
                for _ in 0..=bound {
                    let node = queue.pop_front().unwrap();
                    let node = node.borrow();
                    level_order[i].push(node.val);
                    if let Some(left) = node.left.clone() {
                        queue.push_back(left);
                    }
                    if let Some(right) = node.right.clone() {
                        queue.push_back(right);
                    }
                }
                i += 1;
            }
        }
        level_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root = TreeNode::from(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let expected = vec![vec![3], vec![9, 20], vec![15, 7]];
        assert_eq!(expected, Solution::level_order(root));
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from(vec![Some(1)]);
        let expected = vec![vec![1]];
        assert_eq!(expected, Solution::level_order(root));
    }

    #[test]
    fn case_3() {
        let root = TreeNode::from(vec![]);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(expected, Solution::level_order(root));
    }
}