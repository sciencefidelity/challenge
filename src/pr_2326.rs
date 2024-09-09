use linked::ListNode;

pub struct Solution;

const DIRECTIONS: &[(i32, i32); 4] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, mut head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![-1; usize::try_from(n).unwrap()]; usize::try_from(m).unwrap()];
        let (mut col, mut row, mut mov) = (0, 0, 0);
        while let Some(node) = head {
            result[usize::try_from(row).unwrap()][usize::try_from(col).unwrap()] = node.val;
            head = node.next;
            let r = (row + DIRECTIONS[mov].0 + m) % m;
            let c = (col + DIRECTIONS[mov].1 + n) % n;
            if result[usize::try_from(r).unwrap()][usize::try_from(c).unwrap()] != -1 {
                mov = (mov + 1) % 4;
            }
            row += DIRECTIONS[mov].0;
            col += DIRECTIONS[mov].1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        let head = ListNode::from(&[3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0]);
        let expected = arr![[3, 0, 2, 6, 8], [5, 0, -1, -1, 1], [5, 2, 4, 9, 7]];
        assert_eq!(Solution::spiral_matrix(3, 5, head), expected);
    }

    #[test]
    fn case_2() {
        let head = ListNode::from(&[0, 1, 2]);
        let expected = arr![[0, 1, 2, -1]];
        assert_eq!(Solution::spiral_matrix(1, 4, head), expected);
    }
}
