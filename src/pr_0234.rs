use linked::ListNode;

pub struct Solution;

type List = Option<Box<ListNode>>;

impl Solution {
    pub fn is_palindrome(mut head: List) -> bool {
        let (mut len, mut cur) = (0, &head);
        while let Some(node) = cur {
            cur = &node.next;
            len += 1;
        }
        let mut cur = &mut head;
        for _ in 0..(len - 1) / 2 {
            cur = &mut cur.as_mut().unwrap().next;
        }

        let (mut prev, mut cur) = (None, cur.as_mut().unwrap().next.take());
        while let Some(mut node) = cur {
            cur = node.next;
            node.next = prev;
            prev = Some(node);
        }
        let (mut right, mut left) = (&prev, &head);
        while let (Some(r), Some(l)) = (right, left) {
            if r.val != l.val {
                return false;
            }
            (right, left) = (&r.next, &l.next);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::is_palindrome(ListNode::from(&[1, 2, 2, 1])));
    }

    #[test]
    fn case_2() {
        assert!(!Solution::is_palindrome(ListNode::from(&[1, 2])));
    }
}
