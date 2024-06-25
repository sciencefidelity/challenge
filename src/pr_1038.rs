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

// TODO: test me
