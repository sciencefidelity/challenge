pub struct Solution;

use b_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn balance_bst(root: Tree) -> Tree {
        let mut inorder = Vec::new();
        Self::inorder_traversal(&root, &mut inorder);
        Self::create_balanced_bst(&inorder)
    }

    fn inorder_traversal(root: &Tree, inorder: &mut Vec<i32>) {
        if let Some(root_ref) = root {
            let root_node = root_ref.borrow();
            Self::inorder_traversal(&root_node.left, inorder);
            inorder.push(root_node.val);
            Self::inorder_traversal(&root_node.right, inorder);
        }
    }

    fn create_balanced_bst(inorder: &[i32]) -> Tree {
        if inorder.is_empty() {
            return None;
        }

        let mid = inorder.len() / 2;
        let mut node = TreeNode::new(inorder[mid]);

        node.left = Self::create_balanced_bst(&inorder[..mid]);
        node.right = Self::create_balanced_bst(&inorder[mid + 1..]);

        Some(Rc::new(RefCell::new(node)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let tree = TreeNode::from(vec![
            Some(1),
            None,
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            None,
        ]);
        let solution = TreeNode::from(vec![Some(3), Some(2), Some(4), Some(1)]);
        assert_eq!(Solution::balance_bst(tree), solution);
    }

    #[test]
    fn case_2() {
        let tree = TreeNode::from(vec![Some(2), Some(1), Some(3)]);
        let solution = TreeNode::from(vec![Some(2), Some(1), Some(3)]);
        assert_eq!(Solution::balance_bst(tree), solution);
    }
}
