use b_tree::TreeNode;
use std::{cell::RefCell, mem, rc::Rc};

type Node = Rc<RefCell<TreeNode>>;

struct BSTIterator {
    stack: Vec<Node>,
}

impl BSTIterator {
    fn new(root: Option<Node>) -> Self {
        Self {
            stack: Self::walk_left(vec![], root),
        }
    }

    fn walk_left(mut stack: Vec<Node>, mut root: Option<Node>) -> Vec<Node> {
        while let Some(node) = root {
            let left = node.borrow_mut().left.take();
            stack.push(node);
            root = left;
        }
        stack
    }

    fn next(&mut self) -> i32 {
        let node = self.stack.pop().unwrap();
        let mut node = node.borrow_mut();
        self.stack = Self::walk_left(mem::take(&mut self.stack), node.right.take());
        node.val
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let tree = TreeNode::from(vec![
            Some(7),
            Some(3),
            Some(15),
            None,
            None,
            Some(9),
            Some(20),
        ]);
        let mut bst_iterator = BSTIterator::new(tree);
        assert_eq!(3, bst_iterator.next());
        assert_eq!(7, bst_iterator.next());
        assert!(bst_iterator.has_next());
        assert_eq!(9, bst_iterator.next());
        assert!(bst_iterator.has_next());
        assert_eq!(15, bst_iterator.next());
        assert!(bst_iterator.has_next());
        assert_eq!(20, bst_iterator.next());
        assert!(!bst_iterator.has_next());
    }
}
