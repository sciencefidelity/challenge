use b_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn get_directions(root: Tree, start_value: i32, dest_value: i32) -> String {
        let mut start_path = String::new();
        let mut dest_path = String::new();
        Self::find_path(&root, start_value, &mut start_path);
        Self::find_path(&root, dest_value, &mut dest_path);
        let mut directions = String::with_capacity(start_path.len() + dest_path.len());
        let mut common_path_length = 0;
        while common_path_length < start_path.len()
            && common_path_length < dest_path.len()
            && start_path.chars().nth(common_path_length)
                == dest_path.chars().nth(common_path_length)
        {
            common_path_length += 1;
        }
        for _ in start_path.chars().skip(common_path_length) {
            directions.push('U');
        }
        for c in dest_path.chars().skip(common_path_length) {
            directions.push(c);
        }
        directions
    }

    fn find_path(node: &Tree, target: i32, path: &mut String) -> bool {
        match node {
            None => false,
            Some(n) => {
                if n.borrow().val == target {
                    true
                } else {
                    path.push('L');
                    if Self::find_path(&n.borrow().left, target, path) {
                        return true;
                    }
                    path.remove(path.len() - 1);

                    path.push('R');
                    if Self::find_path(&n.borrow().right, target, path) {
                        return true;
                    }
                    path.remove(path.len() - 1);
                    false
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let tree = TreeNode::from(vec![
            Some(5),
            Some(1),
            Some(2),
            Some(3),
            None,
            Some(6),
            Some(4),
        ]);
        assert_eq!(Solution::get_directions(tree, 3, 6), "UURL".to_owned());
    }

    #[test]
    fn case_2() {
        let tree = TreeNode::from(vec![Some(2), Some(1)]);
        assert_eq!(Solution::get_directions(tree, 2, 1), "L".to_owned());
    }

    #[test]
    fn case_3() {
        let tree = TreeNode::from(vec![
            Some(1),
            None,
            Some(10),
            Some(12),
            Some(13),
            Some(4),
            Some(6),
            None,
            Some(15),
            None,
            None,
            Some(5),
            Some(11),
            None,
            Some(2),
            Some(14),
            Some(7),
            None,
            Some(8),
            None,
            None,
            None,
            Some(9),
            Some(3),
        ]);
        assert_eq!(Solution::get_directions(tree, 6, 15), "UURR".to_owned());
    }
}
