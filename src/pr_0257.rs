use b_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn binary_tree_paths(root: OptNode) -> Vec<String> {
        let mut output = vec![];
        Self::recurse(&root, &String::new(), &mut output);
        output
    }

    fn recurse(maybe_node: &OptNode, path: &String, output: &mut Vec<String>) {
        if let Some(node) = maybe_node {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                output.push(format!("{path}{}", node.val));
            }
            if node.left.is_some() {
                Self::recurse(&node.left, &format!("{path}{}->", node.val), output);
            }
            if node.right.is_some() {
                Self::recurse(&node.right, &format!("{path}{}->", node.val), output);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root = TreeNode::from(vec![Some(1), Some(2), Some(3), None, Some(5)]);
        assert_eq!(
            vec!["1->2->5".to_owned(), "1->3".to_owned()],
            Solution::binary_tree_paths(root)
        );
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from(vec![Some(1)]);
        assert_eq!(vec!["1".to_owned()], Solution::binary_tree_paths(root));
    }

    #[test]
    fn case_3() {
        let root = TreeNode::from(vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
        assert_eq!(
            vec![
                "1->2->4".to_owned(),
                "1->2->5".to_owned(),
                "1->3".to_owned()
            ],
            Solution::binary_tree_paths(root)
        );
    }
}
