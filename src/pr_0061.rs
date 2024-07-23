use linked::ListNode;

pub struct Solution;

type List = Option<Box<ListNode>>;

impl Solution {
    pub fn rotate_right(mut head: List, mut k: i32) -> List {
        let mut len = 0;
        let mut node_ref = head.as_ref();
        while let Some(node) = node_ref {
            len += 1;
            node_ref = node.next.as_ref();
        }
        k %= len;
        if len == 0 || k == 0 {
            return head;
        }
        let mut node_ref = head.as_deref_mut().unwrap();
        for _ in 0..(len - k - 1) {
            node_ref = node_ref.next.as_deref_mut().unwrap();
        }
        let mut new_head = node_ref.next.take().unwrap();
        node_ref = new_head.as_mut();
        while node_ref.next.is_some() {
            node_ref = node_ref.next.as_deref_mut().unwrap();
        }
        node_ref.next = head;
        Some(new_head)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let list = ListNode::from(&[1, 2, 3, 4, 5]);
        let expected = ListNode::from(&[4, 5, 1, 2, 3]);
        assert_eq!(expected, Solution::rotate_right(list, 2));
    }

    #[test]
    fn case_2() {
        let list = ListNode::from(&[0, 1, 2]);
        let expected = ListNode::from(&[2, 0, 1]);
        assert_eq!(expected, Solution::rotate_right(list, 4));
    }
}
