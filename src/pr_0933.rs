use std::collections::VecDeque;

struct RecentCounter {
    pings: VecDeque<i32>,
}

impl RecentCounter {
    const fn new() -> Self {
        Self {
            pings: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        let last_acceptable = t - 3000;
        self.pings.push_back(t);
        while *self.pings.front().unwrap() < last_acceptable {
            self.pings.pop_front();
        }
        i32::try_from(self.pings.len()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut recent_counter = RecentCounter::new();
        assert_eq!(recent_counter.ping(1), 1);
        assert_eq!(recent_counter.ping(100), 2);
        assert_eq!(recent_counter.ping(3001), 3);
        assert_eq!(recent_counter.ping(3002), 3);
    }
}
