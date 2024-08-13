pub struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, mut n: i32) -> bool {
        let mut spaces = 1;
        for flower in flowerbed {
            if flower == 0 {
                spaces += 1;
                if spaces == 3 {
                    n -= 1;
                    spaces = 1;
                }
            } else {
                spaces = 0;
            }
        }
        if spaces == 2 {
            n -= 1;
        }
        n <= 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
    }

    #[test]
    fn case_2() {
        assert!(!Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2));
    }

    #[test]
    fn case_3() {
        assert!(Solution::can_place_flowers(vec![0, 0, 1, 0, 1], 1));
    }

    #[test]
    fn case_4() {
        assert!(Solution::can_place_flowers(vec![1], 0));
    }
}
