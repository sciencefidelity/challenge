use b_tree::TreeNode;
use std::{cell::RefCell, cmp::Ordering, rc::Rc};

pub struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn search_bst(root: Tree, val: i32) -> Tree {
        if let Some(ref root_ref) = root {
            let root_ref = root_ref.borrow();
            match root_ref.val.cmp(&val) {
                Ordering::Equal => return root.clone(),
                Ordering::Greater => return Self::search_bst(root_ref.left.clone(), val),
                Ordering::Less => return Self::search_bst(root_ref.right.clone(), val),
            }
        }
        None
    }
}

// impl Solution {
//     pub fn search_bst(mut root: Tree, val: i32) -> Tree {
//         while let Some(node) = root.clone() {
//             let node = node.borrow();
//             match node.val.cmp(&val) {
//                 Ordering::Equal => return root,
//                 Ordering::Greater => root.clone_from(&node.left),
//                 Ordering::Less => root.clone_from(&node.right),
//             }
//         }
//         None
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root = TreeNode::from(vec![Some(4), Some(2), Some(7), Some(1), Some(3)]);
        let expected = TreeNode::from(vec![Some(2), Some(1), Some(3)]);
        assert_eq!(expected, Solution::search_bst(root, 2));
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from(vec![Some(4), Some(2), Some(7), Some(1), Some(3)]);
        let expected = TreeNode::from(vec![]);
        assert_eq!(expected, Solution::search_bst(root, 5));
    }
}
