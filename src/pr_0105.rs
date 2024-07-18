use b_tree::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::slice::Iter;

pub struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Tree {
        let hm = inorder
            .iter()
            .enumerate()
            .map(|(i, &val)| (val, i))
            .collect::<HashMap<_, _>>();

        Self::builder(
            &mut preorder.iter(),
            &hm,
            (0, isize::try_from(preorder.len()).unwrap() - 1),
        )
    }

    fn builder(
        preorder: &mut Iter<i32>,
        index_map: &HashMap<i32, usize>,
        range: (isize, isize),
    ) -> Tree {
        if range.0 <= range.1 {
            if let Some(&val) = preorder.next() {
                if let Some(&i) = index_map.get(&val) {
                    return Some(Rc::new(RefCell::new(TreeNode {
                        val,
                        left: Self::builder(
                            preorder,
                            index_map,
                            (range.0, isize::try_from(i).unwrap() - 1),
                        ),
                        right: Self::builder(
                            preorder,
                            index_map,
                            (isize::try_from(i).unwrap() + 1, range.1),
                        ),
                    })));
                }
            }
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
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        assert_eq!(Solution::build_tree(preorder, inorder), tree);
    }

    #[test]
    fn case_2() {
        let tree = TreeNode::from(vec![Some(-1)]);
        let preorder = vec![-1];
        let inorder = vec![-1];
        assert_eq!(Solution::build_tree(preorder, inorder), tree);
    }

    #[test]
    fn case_3() {
        let tree = TreeNode::from(vec![Some(1), None, Some(2)]);
        let preorder = vec![1, 2];
        let inorder = vec![1, 2];
        assert_eq!(Solution::build_tree(preorder, inorder), tree);
    }
}
