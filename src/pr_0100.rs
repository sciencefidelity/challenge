use b_tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

// impl Solution {
//     pub fn is_same_tree(p: Tree, q: Tree) -> bool {
//         match (p, q) {
//             (None, None) => true,
//             (Some(p), Some(q)) => {
//                 let (p, q) = (p.borrow(), q.borrow());
//                 p.val == q.val
//                     && Self::is_same_tree(p.left.clone(), q.left.clone())
//                     && Self::is_same_tree(p.right.clone(), q.right.clone())
//             }
//             _ => false,
//         }
//     }
// }

impl Solution {
    pub fn is_same_tree(p: Tree, q: Tree) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back((p, q));
        while !queue.is_empty() {
            match queue.pop_front().unwrap() {
                (None, None) => continue,
                (Some(p), Some(q)) => {
                    let (p, q) = (p.borrow(), q.borrow());
                    if p.val == q.val {
                        queue.push_back((p.left.clone(), q.left.clone()));
                        queue.push_back((p.right.clone(), q.right.clone()));
                    } else {
                        return false;
                    }
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
        let root1 = Rc::new(RefCell::new(TreeNode::new(1)));
        root1.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root1.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

        let root2 = Rc::new(RefCell::new(TreeNode::new(1)));
        root2.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root2.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

        assert_eq!(Solution::is_same_tree(Some(root1), Some(root2)), true);
    }

    #[test]
    fn case_2() {
        let root1 = Rc::new(RefCell::new(TreeNode::new(1)));
        root1.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));

        let root2 = Rc::new(RefCell::new(TreeNode::new(1)));
        root2.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));

        assert_eq!(Solution::is_same_tree(Some(root1), Some(root2)), false);
    }

    #[test]
    fn case_3() {
        let root1 = Rc::new(RefCell::new(TreeNode::new(1)));
        root1.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root1.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(1))));

        let root2 = Rc::new(RefCell::new(TreeNode::new(1)));
        root2.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root2.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));

        assert_eq!(Solution::is_same_tree(Some(root1), Some(root2)), false);
    }
}
