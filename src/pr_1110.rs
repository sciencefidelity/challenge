use b_tree::TreeNode;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn del_nodes(root: Tree, to_delete: Vec<i32>) -> Vec<Tree> {
        let to_delete: HashSet<i32> = HashSet::from_iter(to_delete);
        let mut forest = Vec::new();
        let root = Self::process_node(root, &to_delete, &mut forest);
        if root.is_some() {
            forest.push(root);
        }
        forest
    }

    fn process_node(node: Tree, to_delete: &HashSet<i32>, forest: &mut Vec<Tree>) -> Tree {
        if let Some(node) = node {
            let mut node_ref = node.borrow_mut();
            node_ref.left = Self::process_node(node_ref.left.take(), to_delete, forest);
            node_ref.right = Self::process_node(node_ref.right.take(), to_delete, forest);
            if to_delete.contains(&node_ref.val) {
                if node_ref.left.is_some() {
                    forest.push(node_ref.left.take());
                }
                if node_ref.right.is_some() {
                    forest.push(node_ref.right.take());
                }
                return None;
            }
            return Some(node.clone());
        }
        None
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
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ]);
        let to_delete = vec![3, 5];
        let forest = vec![
            TreeNode::from(vec![Some(1), Some(2), None, Some(4)]),
            TreeNode::from(vec![Some(6)]),
            TreeNode::from(vec![Some(7)]),
        ];
        let solution = Solution::del_nodes(tree, to_delete);
        assert_eq!(solution.len(), 3);
        for tree in forest {
            assert!(solution.contains(&tree));
        }
    }

    #[test]
    fn case_2() {
        let tree = TreeNode::from(vec![Some(1), Some(2), Some(4), None, Some(3)]);
        let to_delete = vec![3];
        let forest = vec![TreeNode::from(vec![Some(1), Some(2), Some(4)])];
        assert_eq!(Solution::del_nodes(tree, to_delete), forest);
    }
}
