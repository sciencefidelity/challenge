use std::collections::BTreeMap;

#[derive(Default)]
struct MyCalendarTwo(BTreeMap<i32, i32>);

impl MyCalendarTwo {
    fn new() -> Self {
        Self::default()
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        *self.0.entry(start).or_insert(0) += 1;
        *self.0.entry(end).or_insert(0) -= 1;
        let mut active = 0;
        for v in self.0.values() {
            active += v;
            if active >= 3 {
                *self.0.entry(start).or_insert(0) -= 1;
                *self.0.entry(end).or_insert(0) += 1;
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut calendar = MyCalendarTwo::new();
        assert!(calendar.book(10, 20));
        assert!(calendar.book(50, 60));
        assert!(calendar.book(10, 40));
        assert!(!calendar.book(5, 15));
        assert!(calendar.book(5, 10));
        assert!(calendar.book(25, 55));
    }
}
