pub struct Solution;

use linked::ListNode;

type List = Option<Box<ListNode>>;

impl Solution {
    pub fn reverse_list(head: List) -> List {
        let (mut prev, mut current) = (None, head);
        while let Some(mut node) = current {
            current = node.next;
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let l1 = ListNode::from(&[1, 2, 3, 4, 5]);
        let l2 = ListNode::from(&[5, 4, 3, 2, 1]);
        assert_eq!(Solution::reverse_list(l1), l2);
    }

    #[test]
    fn case_2() {
        let l1 = ListNode::from(&[1, 2]);
        let l2 = ListNode::from(&[2, 1]);
        assert_eq!(Solution::reverse_list(l1), l2);
    }

    #[test]
    fn case_3() {
        let l1 = ListNode::from(&[]);
        let l2 = ListNode::from(&[]);
        assert_eq!(Solution::reverse_list(l1), l2);
    }
}
