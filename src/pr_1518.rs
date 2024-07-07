pub struct Solution;

impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut consumed_bottles = 0;
        while num_bottles >= num_exchange {
            let k = num_bottles / num_exchange;
            consumed_bottles += num_exchange * k;
            num_bottles -= num_exchange * k;
            num_bottles += k;
        }
        consumed_bottles + num_bottles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::num_water_bottles(9, 3), 13);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::num_water_bottles(15, 4), 19);
    }
}
