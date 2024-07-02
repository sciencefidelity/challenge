pub struct Solution;

use b_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn lowest_common_ancestor(root: Tree, p: Tree, q: Tree) -> Tree {
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;

        let mut res = None;
        let mut res_ptr = &mut res;
        Self::dfs(&root, p_val, q_val, &mut res_ptr);
        res
    }

    fn dfs(node: &Tree, p: i32, q: i32, res_ptr: &mut Tree) {
        match node {
            Some(n_rc) => {
                let n = n_rc.borrow();
                *res_ptr = Some(Rc::new(RefCell::new(TreeNode::new(n.val))));
                if n.val > p && n.val > q {
                    Self::dfs(&n.left, p, q, res_ptr)
                }
                if n.val < p && n.val < q {
                    Self::dfs(&n.right, p, q, res_ptr)
                }
            }
            None => (),
        }
    }
}

// TODO: write tests
