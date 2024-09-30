struct CustomStack {
    values: Vec<(i32, i32)>,
    max_size: usize,
}

impl CustomStack {

    fn new(maxSize: i32) -> Self {
        let max_size = maxSize as usize;
        return Self {
            values: Vec::with_capacity(max_size),
            max_size,
        }
    }

    fn push(&mut self, x: i32) {
        if self.is_full() { return }
        self.values.push((x,0));
    }

    fn pop(&mut self) -> i32 {
        if self.is_emtpy() { return -1 }
        let (val, inc) = self.values.pop().unwrap();
        if let Some(v) = self.values.last_mut() {
            v.1 += inc;
        }
        return val + inc;
    }

    fn increment(&mut self, k: i32, val: i32) {
        if self.is_emtpy() { return }
        let mut k = k as usize;
        k = k.min(self.values.len()) - 1; 
        self.values[k].1 += val;
    }

    fn is_emtpy(&self) -> bool {
        return self.values.len() == 0;
    }

    fn is_full(&self) -> bool {
        return self.values.len() == self.max_size;
    }
}

impl std::fmt::Debug for CustomStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CustomStack:\n  values: {:?}\n", self.values)?;
        return Ok(());
    }
}

#[test]
fn tests() {
    let mut cs = CustomStack::new(5);
    println!("{cs:?}");

    cs.push(2); // 2
    cs.push(2); // 2, 2
    cs.push(3); // 2, 2, 3
    cs.push(4); // 2, 2, 3, 4
    cs.push(7); // 2, 2, 3, 4, 7
    cs.push(5); // shouldn't be pushed
    println!("{cs:?}");

    cs.increment(3, 10); // 12, 12, 13, 4, 7
    println!("{cs:?}");

    cs.increment(9, 5); // 17, 17, 18, 9, 12
    println!("{cs:?}"); 

    println!("popped {}", cs.pop()); // 17, 17, 18, 9, [12]
    println!("{cs:?}"); 

    println!("popped {}", cs.pop()); // 17, 17, 18, [9]
    println!("{cs:?}"); 

}
