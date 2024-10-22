#![allow(clippy::cast_lossless, clippy::cast_sign_loss)]
use b_tree::TreeNode;
use std::collections::{BinaryHeap, VecDeque};
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn kth_largest_level_sum(root: OptNode, k: i32) -> i64 {
        let Some(r) = root else { return -1 };
        let (mut queue, mut heap) = (VecDeque::from([r]), BinaryHeap::new());
        while !queue.is_empty() {
            let sum = (0..queue.len())
                .map(|_| {
                    let n = queue.pop_front().unwrap();
                    let n = n.borrow();
                    if let Some(l) = &n.left {
                        queue.push_back(Rc::clone(l));
                    }
                    if let Some(r) = &n.right {
                        queue.push_back(Rc::clone(r));
                    }
                    n.val as i64
                })
                .sum::<i64>();
            heap.push(-sum);
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        if heap.len() == k as usize {
            -heap.pop().unwrap()
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root = TreeNode::from(vec![
            Some(5),
            Some(8),
            Some(9),
            Some(2),
            Some(1),
            Some(3),
            Some(7),
            Some(4),
            Some(6),
        ]);
        assert_eq!(Solution::kth_largest_level_sum(root, 2), 13);
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from(vec![Some(1), Some(2), None, Some(3)]);
        assert_eq!(Solution::kth_largest_level_sum(root, 1), 3);
    }
}
