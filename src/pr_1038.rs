pub struct Solution;

use b_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn bst_to_gst(root: Tree) -> Tree {
        Self::dfs(&root, &mut 0);
        root
    }

    fn dfs(root: &Tree, node_sum: &mut i32) {
        if let Some(root_ref) = root {
            let mut root_node = root_ref.borrow_mut();
            Self::dfs(&root_node.right, node_sum);
            *node_sum += root_node.val;
            root_node.val = *node_sum;
            Self::dfs(&root_node.left, node_sum);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let tree = TreeNode::from(vec![
            Some(4),
            Some(1),
            Some(6),
            Some(0),
            Some(2),
            Some(5),
            Some(7),
            None,
            None,
            None,
            Some(3),
            None,
            None,
            None,
            Some(8),
        ]);
        let solution = TreeNode::from(vec![
            Some(30),
            Some(36),
            Some(21),
            Some(36),
            Some(35),
            Some(26),
            Some(15),
            None,
            None,
            None,
            Some(33),
            None,
            None,
            None,
            Some(8),
        ]);
        assert_eq!(Solution::bst_to_gst(tree), solution);
    }

    #[test]
    fn case_2() {
        let tree = TreeNode::from(vec![Some(0), None, Some(1)]);
        let solution = TreeNode::from(vec![Some(1), None, Some(1)]);
        assert_eq!(Solution::bst_to_gst(tree), solution);
    }
}
