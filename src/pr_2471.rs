use b_tree::TreeNode;
use std::collections::{HashMap, VecDeque};
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn minimum_operations(root: OptNode) -> i32 {
        let mut total_swaps = 0;
        if let Some(node_) = root {
            let mut queue = VecDeque::new();
            queue.push_back(node_);
            while !queue.is_empty() {
                let mut level_values = vec![0; queue.len()];
                for val in &mut level_values {
                    let node = queue.pop_front().unwrap();
                    let node = node.borrow();
                    *val = node.val;
                    if let Some(left) = node.left.clone() {
                        queue.push_back(left);
                    }
                    if let Some(right) = node.right.clone() {
                        queue.push_back(right);
                    }
                }
                total_swaps += Self::get_min_swaps(&mut level_values);
            }
        }
        total_swaps
    }

    fn get_min_swaps(original: &mut [i32]) -> i32 {
        let mut swaps = 0;
        let mut target = original.to_owned();
        target.sort_unstable();
        let mut pos = HashMap::new();
        for (i, val) in original.iter().enumerate() {
            pos.insert(*val, i);
        }
        for i in 0..original.len() {
            if original[i] != target[i] {
                swaps += 1;
                let &cur_pos = pos.get(&target[i]).unwrap();
                pos.insert(original[i], cur_pos);
                original.swap(cur_pos, i);
            }
        }
        swaps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root = TreeNode::from(vec![
            Some(1),
            Some(4),
            Some(3),
            Some(7),
            Some(6),
            Some(8),
            Some(5),
            None,
            None,
            None,
            None,
            Some(9),
            None,
            Some(10),
        ]);
        assert_eq!(Solution::minimum_operations(root), 3);
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from(vec![
            Some(1),
            Some(3),
            Some(2),
            Some(7),
            Some(6),
            Some(5),
            Some(4),
        ]);
        assert_eq!(Solution::minimum_operations(root), 3);
    }

    #[test]
    fn case_3() {
        let root = TreeNode::from(vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
        assert_eq!(Solution::minimum_operations(root), 0);
    }
}
