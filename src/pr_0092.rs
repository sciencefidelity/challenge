use linked::ListNode;

pub struct Solution;

type List = Option<Box<ListNode>>;

impl Solution {
    pub fn reverse_between(mut head: List, left: i32, right: i32) -> List {
        if head.is_none() || left == right {
            return head;
        }
        let mut next: Option<Box<ListNode>>;
        let mut prev: Option<Box<ListNode>> = None;
        let mut current: Option<&mut Box<ListNode>> = None;

        if left == 1 {
            next = head.take();
        } else {
            current = head.as_mut();
            for _ in 1..(left - 1) {
                current = current.unwrap().next.as_mut();
            }
            next = current.as_mut().unwrap().next.take();
        }

        for _ in 0..=(right - left) {
            prev = std::mem::replace(&mut next.as_mut().unwrap().next, prev);
            std::mem::swap(&mut next, &mut prev);
        }

        if let Some(n) = current {
            n.next = prev;
            current = n.next.as_mut();
        } else {
            head = prev;
            current = head.as_mut();
        }

        for _ in 0..(right - left) {
            current = current.unwrap().next.as_mut();
        }

        current.unwrap().next = next;

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let list = ListNode::from(&[1, 2, 3, 4, 5]);
        let expected = ListNode::from(&[1, 4, 3, 2, 5]);
        assert_eq!(Solution::reverse_between(list, 2, 4), expected);
    }

    #[test]
    fn case_2() {
        let list = ListNode::from(&[5]);
        let expected = ListNode::from(&[5]);
        assert_eq!(Solution::reverse_between(list, 1, 1), expected);
    }
}
