pub struct Solution;

use linked::ListNode;

type List = Option<Box<ListNode>>;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn middle_node(head: List) -> List {
        let (mut slow, mut fast) = (&head, &head);
        while let Some(ListNode {
            next: Some(node), ..
        }) = fast.as_deref()
        {
            slow = &slow.as_ref().unwrap().next;
            fast = &node.next;
        }
        slow.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let l1 = ListNode::from(vec![1, 2, 3, 4, 5]);
        let l2 = ListNode::from(vec![3, 4, 5]);
        assert_eq!(Solution::middle_node(l1), l2);
    }

    #[test]
    fn case_2() {
        let l1 = ListNode::from(vec![1, 2, 3, 4, 5, 6]);
        let l2 = ListNode::from(vec![4, 5, 6]);
        assert_eq!(Solution::middle_node(l1), l2);
    }
}
