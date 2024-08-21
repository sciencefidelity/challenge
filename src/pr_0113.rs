use b_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn path_sum(root: OptNode, target_sum: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        Self::dfs(&root, target_sum, &mut result, &mut Vec::new());
        result
    }

    fn dfs(
        maybe_node: &OptNode,
        target_sum: i32,
        result: &mut Vec<Vec<i32>>,
        path_vals: &mut Vec<i32>,
    ) {
        if let Some(node) = maybe_node {
            let node = node.borrow();
            let val = node.val;
            path_vals.push(node.val);
            if target_sum == val && node.left.is_none() && node.right.is_none() {
                result.push(path_vals.clone());
            }
            Self::dfs(&node.left, target_sum - val, result, path_vals);
            Self::dfs(&node.right, target_sum - val, result, path_vals);
            path_vals.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
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
        assert_eq!(
            Solution::path_sum(root, 22),
            arr![[5, 4, 11, 2], [5, 8, 4, 5]]
        );
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from(vec![Some(1), Some(2), Some(3)]);
        assert_eq!(Solution::path_sum(root, 5), Vec::<Vec<i32>>::new());
    }
}
