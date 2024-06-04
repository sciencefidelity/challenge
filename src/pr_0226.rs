pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use b_tree::TreeNode;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn invert_tree(root: Tree) -> Tree {
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let tree = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        assert_eq!(Solution::invert_tree(tree.clone()), tree);
    }

    #[test]
    fn case_2() {
        let tree = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        assert_eq!(Solution::invert_tree(tree.clone()), tree);
    }

    #[test]
    fn case_3() {
        let tree = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        assert_eq!(Solution::invert_tree(tree.clone()), tree);
    }
}
