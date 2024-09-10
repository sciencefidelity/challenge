use linked::ListNode;

pub struct Solution;

type OptNode = Option<Box<ListNode>>;

impl Solution {
    pub fn insert_greatest_common_divisors(head: OptNode) -> OptNode {
        let mut head = head?;
        let mut curr = &mut *head;
        while let Some(node) = curr.next.take() {
            let new_node = Box::new(ListNode::new(Self::gcd(curr.val, node.val)));
            curr = curr.next.insert(new_node).next.insert(node);
        }
        head.into()
    }

    pub const fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let head = ListNode::from(&[18, 6, 10, 3]);
        let expected = ListNode::from(&[18, 6, 6, 2, 10, 1, 3]);
        assert_eq!(Solution::insert_greatest_common_divisors(head), expected);
    }

    #[test]
    fn case_2() {
        let head = ListNode::from(&[7]);
        let expected = ListNode::from(&[7]);
        assert_eq!(Solution::insert_greatest_common_divisors(head), expected);
    }
}
