struct CustomStack {
    stack: Vec<i32>,
    max_size: usize,
}

impl CustomStack {
    fn new(max_size: i32) -> Self {
        let max_size = usize::try_from(max_size).unwrap();
        Self {
            stack: Vec::with_capacity(max_size),
            max_size,
        }
    }

    fn push(&mut self, x: i32) {
        if self.stack.len() < self.max_size {
            self.stack.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        if self.stack.is_empty() {
            -1
        } else {
            self.stack.pop().unwrap()
        }
    }

    fn increment(&mut self, k: i32, val: i32) {
        for num in self.stack.iter_mut().take(usize::try_from(k).unwrap()) {
            *num += val
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut stk = CustomStack::new(3);
        stk.push(1);
        stk.push(2);
        assert_eq!(stk.pop(), 2);
        stk.push(2);
        stk.push(3);
        stk.push(4);
        stk.increment(5, 100);
        stk.increment(2, 100);
        stk.pop();
        stk.pop();
        stk.pop();
        stk.pop();
    }
}
