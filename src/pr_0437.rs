use b_tree::TreeNode;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Solution;

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn path_sum(root: OptNode, target_sum: i32) -> i32 {
        let (mut pre_sum, mut result) = (HashMap::from([(0, 1)]), 0);
        Self::dfs(&root, target_sum, 0, &mut pre_sum, &mut result);
        result
    }

    fn dfs(
        maybe_node: &OptNode,
        target: i32,
        curr_sum: i32,
        pre_sum: &mut HashMap<i32, i32>,
        result: &mut i32,
    ) {
        if let Some(node) = maybe_node {
            let val = node.borrow().val;
            let curr = {
                if curr_sum.checked_add(val).is_some() {
                    curr_sum + val
                } else {
                    pre_sum.clear();
                    pre_sum.insert(0, 1);
                    val
                }
            };
            let prev = curr - target;
            if let Some(num) = pre_sum.get(&prev) {
                *result += num;
            }
            pre_sum.entry(curr).and_modify(|e| *e += 1).or_insert(1);

            Self::dfs(&node.borrow().left, target, curr, pre_sum, result);
            Self::dfs(&node.borrow().right, target, curr, pre_sum, result);
            pre_sum.entry(curr).and_modify(|e| *e -= 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root = TreeNode::from(vec![
            Some(10),
            Some(5),
            Some(-3),
            Some(3),
            Some(2),
            None,
            Some(11),
            Some(3),
            Some(-2),
            None,
            Some(1),
        ]);
        assert_eq!(Solution::path_sum(root, 8), 3);
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from(vec![
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
            Some(5),
            Some(1),
        ]);
        assert_eq!(Solution::path_sum(root, 22), 3);
    }
}
