struct MyCircularDeque {
    array: Vec<i32>,
    front: i32,
    rear: i32,
    size: i32,
    capacity: i32,
}

impl MyCircularDeque {
    fn new(k: i32) -> Self {
        let ku = usize::try_from(k).unwrap();
        Self {
            array: vec![-1; ku],
            front: 0,
            rear: k - 1,
            size: 0,
            capacity: k,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.front = (self.front - 1 + self.capacity) % self.capacity;
        self.array[usize::try_from(self.front).unwrap()] = value;
        self.size += 1;
        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.rear = (self.rear + 1) % self.capacity;
        self.array[usize::try_from(self.rear).unwrap()] = value;
        self.size += 1;
        true
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
        true
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.rear = (self.rear - 1 + self.capacity) % self.capacity;
        self.size -= 1;
        true
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.array[usize::try_from(self.front).unwrap()]
        }
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.array[usize::try_from(self.rear).unwrap()]
        }
    }

    const fn is_empty(&self) -> bool {
        self.size == 0
    }

    const fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut obj = MyCircularDeque::new(3);
        assert!(obj.insert_last(1));
        assert!(obj.insert_last(2));
        assert!(obj.insert_front(3));
        assert!(!obj.insert_front(4));
        assert_eq!(obj.get_rear(), 2);
        assert!(obj.is_full());
        assert!(obj.delete_last());
        assert!(obj.insert_front(4));
        assert_eq!(obj.get_front(), 4);
    }
}
