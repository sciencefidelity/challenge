use std::collections::BTreeMap;

struct MyCalendar(BTreeMap<i32, i32>);

impl MyCalendar {
    const fn new() -> Self {
        Self(BTreeMap::new())
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        if let Some((_, &e)) = self.0.range(..end).next_back() {
            if start < e {
                return false;
            }
        }
        self.0.insert(start, end);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut calendar = MyCalendar::new();
        assert!(calendar.book(10, 20));
        assert!(!calendar.book(15, 25));
        assert!(calendar.book(20, 30));
    }
}
