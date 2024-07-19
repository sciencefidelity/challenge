use b_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Tree {
        Self::builder(&inorder, &postorder)
    }

    fn builder(inorder: &[i32], postorder: &[i32]) -> Tree {
        if inorder.is_empty() || postorder.is_empty() {
            return None;
        }
        let i = inorder.len();
        let n = postorder.len() - 1;
        let mut root = TreeNode::new(postorder[n]);
        let m = inorder.iter().position(|&x| x == postorder[n]).unwrap();

        root.left = Self::builder(&inorder[..m], &postorder[..m]);
        root.right = Self::builder(&inorder[(m + 1)..i], &postorder[m..(i - 1)]);
        Some(Rc::new(RefCell::new(root)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let tree = TreeNode::from(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let inorder = vec![9, 3, 15, 20, 7];
        let postorder = vec![9, 15, 7, 20, 3];
        assert_eq!(Solution::build_tree(inorder, postorder), tree);
    }

    #[test]
    fn case_2() {
        let tree = TreeNode::from(vec![Some(-1)]);
        let postorder = vec![-1];
        let inorder = vec![-1];
        assert_eq!(Solution::build_tree(inorder, postorder), tree);
    }
}
