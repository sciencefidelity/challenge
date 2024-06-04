pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use b_tree::TreeNode;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn invert_tree(root: Tree) -> Tree {
        if let Some(root) = &root {
            if let Ok(mut root) = root.try_borrow_mut() {
                let right = root.right.clone();
                root.right = Solution::invert_tree(root.left.clone());
                root.left = Solution::invert_tree(right);
            }
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        todo!();
        // assert_eq!(Solution::invert_tree(root.clone()), root);
    }

    #[test]
    fn case_2() {
        todo!();
        // assert_eq!(Solution::invert_tree(tree.clone()), tree);
    }

    #[test]
    fn case_3() {
        let root = None;
        assert_eq!(Solution::invert_tree(root), None);
    }
}
