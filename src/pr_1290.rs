use linked::ListNode;

pub struct Solution;

impl Solution {
    pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
        let mut result = 0;
        while let Some(node) = head {
            result <<= 1;
            result |= node.val;
            head = node.next;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::get_decimal_value(ListNode::from(&[1, 0, 1])), 5);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::get_decimal_value(ListNode::from(&[0])), 0);
    }
}
