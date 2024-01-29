struct MyQueue {
    s1: Vec<i32>,
    s2: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Self { s1: Vec::new(), s2: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        self.s1.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.s2.is_empty() {
            self.fill_s2();
        }
        return self.s2.pop().unwrap();
    }

    fn peek(&mut self) -> i32 {
        if self.s2.is_empty() {
            self.fill_s2();
        }
        return *self.s2.last().unwrap();
    }

    fn empty(&self) -> bool {
        if self.s1.is_empty() && self.s2.is_empty() {
            return true;
        }
        return false;
    }

    fn fill_s2(&mut self) {
        while let Some(_) = self.s1.last() {
            let v = self.s1.pop().unwrap();
            self.s2.push(v);
        }
    }
}
