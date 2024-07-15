#[derive(Default)]
struct MyQueue {
    stack_in: Vec<i32>,
    stack_out: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Self::default()
    }

    fn transfer(&mut self) {
        if self.stack_out.is_empty() {
            while let Some(x) = self.stack_in.pop() {
                self.stack_out.push(x);
            }
        }
    }

    fn push(&mut self, x: i32) {
        self.stack_in.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.transfer();
        self.stack_out.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        self.transfer();
        *self.stack_out.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack_in.is_empty() && self.stack_out.is_empty()
    }
}

// TODO: write tests
