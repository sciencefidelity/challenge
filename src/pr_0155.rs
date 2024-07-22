use std::cmp::min;

struct MinStack {
    stack: Vec<(i32, i32)>,
    min: i32,
}

impl MinStack {
    pub const fn new() -> Self {
        Self {
            stack: Vec::new(),
            min: i32::MAX,
        }
    }

    pub fn push(&mut self, val: i32) {
        let elem = (val, self.min);
        self.stack.push(elem);
        self.min = min(self.min, val);
    }

    pub fn pop(&mut self) {
        self.min = self.stack.pop().unwrap().1;
    }

    pub fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    pub const fn get_min(&self) -> i32 {
        self.min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }
}
