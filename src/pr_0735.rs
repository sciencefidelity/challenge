use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn asteroid_collision(mut asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::with_capacity(asteroids.len());
        for a in &mut asteroids {
            while !stack.is_empty() && *a < 0 && stack[stack.len() - 1] > 0 {
                let diff: i32 = *a + stack[stack.len() - 1];
                match diff.cmp(&0) {
                    Ordering::Less => {
                        stack.pop();
                    }
                    Ordering::Greater => *a = 0,
                    Ordering::Equal => {
                        *a = 0;
                        stack.pop();
                    }
                }
            }
            if *a != 0 {
                stack.push(*a);
            }
        }
        stack
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(vec![5, 10], Solution::asteroid_collision(vec![5, 10, -5]));
    }

    #[test]
    fn case_2() {
        assert_eq!(Vec::<i32>::new(), Solution::asteroid_collision(vec![8, -8]));
    }

    #[test]
    fn case_3() {
        assert_eq!(vec![10], Solution::asteroid_collision(vec![10, 2, -5]));
    }

    #[test]
    fn case_4() {
        assert_eq!(
            vec![-5, -10, 5],
            Solution::asteroid_collision(vec![-5, -10, 5])
        );
    }
}
