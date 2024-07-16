use b_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn lowest_common_ancestor(root: Tree, p: Tree, q: Tree) -> Tree {
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;

        let mut res = None;
        let res_ptr = &mut res;
        Self::dfs(&root, p_val, q_val, res_ptr);
        res
    }

    fn dfs(node: &Tree, p: i32, q: i32, res_ptr: &mut Tree) {
        match node {
            Some(n_rc) => {
                let n = n_rc.borrow();
                *res_ptr = Some(Rc::new(RefCell::new(TreeNode::new(n.val))));
                if n.val > p && n.val > q {
                    Self::dfs(&n.left, p, q, res_ptr);
                }
                if n.val < p && n.val < q {
                    Self::dfs(&n.right, p, q, res_ptr);
                }
            }
            None => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let tree = TreeNode::from(vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        let p = TreeNode::from(vec![Some(2)]);
        let q = TreeNode::from(vec![Some(8)]);
        let solution = TreeNode::from(vec![Some(6)]);
        assert_eq!(Solution::lowest_common_ancestor(tree, p, q), solution);
    }

    #[test]
    fn case_2() {
        let tree = TreeNode::from(vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        let p = TreeNode::from(vec![Some(2)]);
        let q = TreeNode::from(vec![Some(4)]);
        let solution = TreeNode::from(vec![Some(2)]);
        assert_eq!(Solution::lowest_common_ancestor(tree, p, q), solution);
    }

    #[test]
    fn case_3() {
        let tree = TreeNode::from(vec![Some(2), Some(1)]);
        let p = TreeNode::from(vec![Some(2)]);
        let q = TreeNode::from(vec![Some(1)]);
        let solution = TreeNode::from(vec![Some(2)]);
        assert_eq!(Solution::lowest_common_ancestor(tree, p, q), solution);
    }
}
