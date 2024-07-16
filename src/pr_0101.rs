use b_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

// impl Solution {
//     #[allow(clippy::needless_pass_by_value)]
//     pub fn is_symmetric(root: Tree) -> bool {
//         Self::dfs(&root, &root)
//     }
//
//     fn dfs(n1: &Tree, n2: &Tree) -> bool {
//         match (n1, n2) {
//             (None, None) => true,
//             (Some(n1), Some(n2)) => {
//                 let (n1, n2) = (n1.borrow(), n2.borrow());
//                 n1.val == n2.val && Self::dfs(&n1.left, &n2.right) && Self::dfs(&n1.right, &n2.left)
//             }
//             _ => false,
//         }
//     }
// }

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn is_symmetric(root: Tree) -> bool {
        if root.is_none() {
            return true;
        }
        let root = root.as_ref().unwrap().borrow();
        let mut stack = vec![(root.left.clone(), root.right.clone())];
        while let Some(tuple) = stack.pop() {
            match tuple {
                (None, None) => continue,
                (Some(n1), Some(n2)) => {
                    let (n1, n2) = (n1.borrow(), n2.borrow());
                    if n1.val != n2.val {
                        return false;
                    }
                    stack.push((n1.left.clone(), n2.right.clone()));
                    stack.push((n1.right.clone(), n2.left.clone()));
                }
                _ => return false,
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let tree = TreeNode::from(vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3),
        ]);
        assert_eq!(Solution::is_symmetric(tree), true);
    }

    #[test]
    fn case_2() {
        let tree = TreeNode::from(vec![
            Some(1),
            Some(2),
            Some(2),
            None,
            Some(3),
            None,
            Some(3),
        ]);
        assert_eq!(Solution::is_symmetric(tree), false);
    }
}
