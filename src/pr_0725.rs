use linked::ListNode;

pub struct Solution;

type OptNode = Option<Box<ListNode>>;

impl Solution {
    pub fn split_list_to_parts(mut head: OptNode, k: i32) -> Vec<OptNode> {
        let (mut n, mut curr, k) = (0, &head, usize::try_from(k).unwrap());
        while let Some(node) = curr {
            n += 1;
            curr = &node.next;
        }
        let (split_size, rem_parts) = (n / k, n % k);
        let mut result = Vec::with_capacity(k);
        for i in 0..k {
            result.push(head);
            let mut node = &mut result[i];
            for _ in 0..split_size + usize::from(i < rem_parts) {
                if let Some(n) = node {
                    node = &mut n.next;
                }
            }
            head = node.take();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let head = ListNode::from(&[1, 2, 3]);
        let expected = vec![
            ListNode::from(&[1]),
            ListNode::from(&[2]),
            ListNode::from(&[3]),
            ListNode::from(&[]),
            ListNode::from(&[]),
        ];
        assert_eq!(Solution::split_list_to_parts(head, 5), expected);
    }

    #[test]
    fn case_2() {
        let head = ListNode::from(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let expected = vec![
            ListNode::from(&[1, 2, 3, 4]),
            ListNode::from(&[5, 6, 7]),
            ListNode::from(&[8, 9, 10]),
        ];
        assert_eq!(Solution::split_list_to_parts(head, 3), expected);
    }
}
