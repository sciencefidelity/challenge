use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    unsafe fn guess_number(mut n: i32) -> i32 {
        let (mut low, mut high) = (0, n);
        loop {
            n = (high - low) / 2 + low;
            match guess(n) {
                0 => break,
                1 => low = n + 1,
                -1 => high = n - 1,
                _ => unreachable!(),
            }
        }
        n
    }
}

unsafe fn guess(guess: i32) -> i32 {
    let num = 6;
    match guess.cmp(&num) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(unsafe { Solution::guess_number(10) }, 6);
    }
}
