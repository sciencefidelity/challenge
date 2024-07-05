pub struct Solution;

use linked::ListNode;

impl Solution {
    pub fn nodes_between_critical_points(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut previous = None;
        let mut min_distance = i32::MAX;
        let mut current_index = 0;
        let mut first_critical_index = None;
        let mut last_critical_index = None;

        while let Some(next) = head {
            if let (Some(prev), Some(next_next)) = (previous, &next.next) {
                if (next.val > prev && next.val > next_next.val)
                    || (next.val < prev && next.val < next_next.val)
                {
                    if first_critical_index.is_none() {
                        first_critical_index = Some(current_index);
                        last_critical_index = Some(current_index);
                    } else {
                        if let Some(last) = last_critical_index {
                            min_distance = min_distance.min(current_index - last);
                        }
                        last_critical_index = Some(current_index);
                    }
                }
            }
            previous = Some(next.val);
            head = next.next;
            current_index += 1;
        }
        if first_critical_index.is_none()
            || last_critical_index.is_none()
            || last_critical_index == first_critical_index
        {
            return vec![-1, -1];
        }
        vec![
            min_distance,
            last_critical_index.unwrap() - first_critical_index.unwrap(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let list = ListNode::from(vec![3, 1]);
        assert_eq!(Solution::nodes_between_critical_points(list), vec![-1, -1]);
    }

    #[test]
    fn case_2() {
        let list = ListNode::from(vec![5, 3, 1, 2, 5, 1, 2]);
        assert_eq!(Solution::nodes_between_critical_points(list), vec![1, 3]);
    }

    #[test]
    fn case_3() {
        let list = ListNode::from(vec![1, 3, 2, 2, 3, 2, 2, 2, 7]);
        assert_eq!(Solution::nodes_between_critical_points(list), vec![3, 3]);
    }
}
