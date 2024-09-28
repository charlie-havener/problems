struct MyCircularDeque {
    values: Vec<Option<i32>>,
    head: usize,
    tail: usize,
}

impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self {
            values: vec![None; k as usize],
            head: 0,
            tail: 0,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() { return false }
        if self.is_empty() {
            self.values[self.head] = Some(value);
            return true;
        }
        let next_idx = (self.head + self.values.len() - 1).rem_euclid(self.values.len());
        self.values[next_idx] = Some(value);
        self.head = next_idx;
        return true;
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() { return false }
        if self.is_empty() {
            self.values[self.tail] = Some(value);
            return true;
        }
        let next_idx = (self.tail + 1).rem_euclid(self.values.len());
        self.values[next_idx] = Some(value);
        self.tail = next_idx;
        return true;
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() { return false }
        let next_idx = (self.head + 1).rem_euclid(self.values.len());
        self.values[self.head] = None;
        if let Some(_) =  self.values[next_idx] {
            self.head = next_idx;
        }
        return true;
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() { return false }
        let next_idx = (self.tail + self.values.len() - 1).rem_euclid(self.values.len());
        self.values[self.tail] = None;
        if let Some(_) = self.values[next_idx] {
            self.tail = next_idx;
        }
        return true;
    }

    fn get_front(&self) -> i32 {
        return self.values[self.head].unwrap_or(-1);

    }

    fn get_rear(&self) -> i32 {
        return self.values[self.tail].unwrap_or(-1);
    }

    fn is_empty(&self) -> bool {
        return self.head == self.tail && self.values[self.head].is_none();
    }

    fn is_full(&self) -> bool {
        let head_next = (self.head + self.values.len() - 1).rem_euclid(self.values.len());
        if head_next == self.tail { return true }
        return false;
    }
}

#[test]
fn tests() {
    let mut s = MyCircularDeque::new(3);
    assert!(s.insert_last(1));
    assert!(s.insert_last(2));
    assert!(s.insert_front(3));
    assert!(!s.insert_front(4));
    assert_eq!(s.get_rear(), 2);
    assert!(s.is_full());
    assert!(s.delete_last());
    assert!(s.insert_front(4));
    assert_eq!(s.get_front(), 4);

    assert_eq!(vec![Some(1),Some(4),Some(3)], s.values);
    assert_eq!(0, s.tail);
    assert_eq!(1, s.head);
}
