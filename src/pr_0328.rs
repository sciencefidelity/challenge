use linked::ListNode;

pub struct Solution;

impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut odd, mut even) = (Box::new(ListNode::new(-1)), Box::new(ListNode::new(-1)));
        let (mut cur_odd, mut cur_even, mut is_odd) = (&mut odd, &mut even, true);
        while head.is_some() {
            if is_odd {
                cur_odd.next = head;
                cur_odd = cur_odd.next.as_mut()?;
                head = cur_odd.next.take();
            } else {
                cur_even.next = head;
                cur_even = cur_even.next.as_mut()?;
                head = cur_even.next.take();
            }
            is_odd = !is_odd;
        }
        cur_odd.next = even.next;
        odd.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let head = ListNode::from(&[1, 2, 3, 4, 5]);
        let expected = ListNode::from(&[1, 3, 5, 2, 4]);
        assert_eq!(expected, Solution::odd_even_list(head));
    }

    #[test]
    fn case_2() {
        let head = ListNode::from(&[2, 1, 3, 5, 6, 4, 7]);
        let expected = ListNode::from(&[2, 3, 6, 7, 1, 5, 4]);
        assert_eq!(expected, Solution::odd_even_list(head));
    }
}
