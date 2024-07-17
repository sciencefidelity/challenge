use b_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

// impl Solution {
//     pub fn count_nodes(root: Tree) -> i32 {
//         root.map_or(0, |node| {
//             let node = node.borrow();
//             1 + Self::count_nodes(node.left.clone()) + Self::count_nodes(node.right.clone())
//         })
//     }
// }

impl Solution {
    pub fn count_nodes(root: Tree) -> i32 {
        let height = Self::height(&root);

        root.map_or(0, |root| {
            if height < 0 {
                0
            } else {
                let root = root.borrow();
                if Self::height(&root.right.clone()) == height - 1 {
                    (1 << height) + Self::count_nodes(root.right.clone())
                } else {
                    (1 << (height - 1)) + Self::count_nodes(root.left.clone())
                }
            }
        })
    }

    fn height(root: &Tree) -> i32 {
        root.as_ref().map_or(-1, |node| {
            let root = node.borrow();
            1 + Self::height(&root.left.clone())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let tree = TreeNode::from(vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
        assert_eq!(Solution::count_nodes(tree), 6);
    }

    #[test]
    fn case_2() {
        let tree = TreeNode::from(vec![]);
        assert_eq!(Solution::count_nodes(tree), 0);
    }

    #[test]
    fn case_3() {
        let tree = TreeNode::from(vec![Some(1)]);
        assert_eq!(Solution::count_nodes(tree), 1);
    }
}
