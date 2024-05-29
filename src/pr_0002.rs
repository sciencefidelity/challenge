use linked::ListNode;

pub struct Solution;

type List = Option<Box<ListNode>>;

impl Solution {
    pub fn add_two_numbers(l1: List, l2: List) -> List {
        return Self::add_two_numbers_recursive(l1, l2, 0);
    }

    pub fn add_two_numbers_recursive(l1: List, l2: List, carry: i32) -> List {
        match (l1, l2) {
            (None, None) => {
                if carry == 1 {
                    Some(Box::new(ListNode::new(1)))
                } else {
                    None
                }
            }
            (Some(n), None) => Some(Box::new(ListNode {
                val: (n.val + carry) % 10,
                next: Self::add_two_numbers_recursive(n.next, None, (n.val + carry) / 10),
            })),
            (None, Some(n)) => Some(Box::new(ListNode {
                val: (n.val + carry) % 10,
                next: Self::add_two_numbers_recursive(None, n.next, (n.val + carry) / 10),
            })),
            (Some(l), Some(r)) => Some(Box::new(ListNode {
                val: (l.val + r.val + carry) % 10,
                next: Self::add_two_numbers_recursive(l.next, r.next, (l.val + r.val + carry) / 10),
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let l1 = ListNode::from(vec![2, 4, 3]);
        let l2 = ListNode::from(vec![5, 6, 4]);
        let list = ListNode::from(vec![7, 0, 8]);
        assert_eq!(list, Solution::add_two_numbers(l1, l2));
    }

    #[test]
    fn case_2() {
        let l1 = ListNode::from(vec![0]);
        let l2 = ListNode::from(vec![0]);
        let list = ListNode::from(vec![0]);
        assert_eq!(list, Solution::add_two_numbers(l1, l2));
    }

    #[test]
    fn case_3() {
        let l1 = ListNode::from(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = ListNode::from(vec![9, 9, 9, 9]);
        let list = ListNode::from(vec![8, 9, 9, 9, 0, 0, 0, 1]);
        assert_eq!(list, Solution::add_two_numbers(l1, l2));
    }
}
