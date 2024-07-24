use b_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn average_of_levels(root: Tree) -> Vec<f64> {
        let mut average = Vec::new();
        Self::recurse(&root, 0, &mut average, &mut Vec::new());
        average
    }

    fn recurse(node: &Tree, depth: usize, average: &mut Vec<f64>, count: &mut Vec<f64>) {
        if let Some(node) = node {
            if depth == average.len() {
                average.push(0.0);
                count.push(0.0);
            }
            let node = node.borrow();
            average[depth] =
                average[depth].mul_add(count[depth], f64::from(node.val)) / (count[depth] + 1.0);
            count[depth] += 1.0;
            Self::recurse(&node.left, depth + 1, average, count);
            Self::recurse(&node.right, depth + 1, average, count);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root = TreeNode::from(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let expected = vec![3.0, 14.5, 11.0];
        assert_eq!(expected, Solution::average_of_levels(root));
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from(vec![Some(3), Some(9), Some(20), Some(15), Some(7)]);
        let expected = vec![3.0, 14.5, 11.0];
        assert_eq!(expected, Solution::average_of_levels(root));
    }

    #[test]
    fn case_3() {
        let root = TreeNode::from(vec![
            Some(3),
            Some(1),
            Some(5),
            Some(0),
            Some(2),
            Some(4),
            Some(6),
        ]);
        let expected = vec![3.0, 3.0, 3.0];
        assert_eq!(expected, Solution::average_of_levels(root));
    }
}
