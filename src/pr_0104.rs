use b_tree::TreeNode;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

pub struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn max_depth(root: Tree) -> i32 {
        root.map_or(0, |node| {
            cmp::max(
                Self::max_depth(node.borrow().left.clone()),
                Self::max_depth(node.borrow().right.clone()),
            ) + 1
        })
    }
}

// impl Solution {
//     pub fn max_depth(root: Tree) -> i32 {
//         let mut max_depth = 0;
//         let mut stack = vec![(root, 0)];
//         while let Some((node, mut depth)) = stack.pop() {
//             if let Some(node) = node {
//                 let node = node.borrow();
//                 depth += 1;
//                 max_depth = max_depth.max(depth);
//
//                 stack.push((node.left.clone(), depth));
//                 stack.push((node.right.clone(), depth));
//             }
//         }
//         max_depth
//     }
// }

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
        assert_eq!(Solution::max_depth(tree), 3);
    }

    #[test]
    fn case_2() {
        let tree = TreeNode::from(vec![Some(1), None, Some(2)]);
        assert_eq!(Solution::max_depth(tree), 2);
    }
}
