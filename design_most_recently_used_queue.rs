#[derive(Debug)]
struct MRUQueue {
    bit: [i32; 4001], //fenwick tree
    lookup: [i32; 4001], // store the value at each index
    size: i32, // track curren max pos
}

impl MRUQueue {
    
    fn new(n: i32) -> Self {
        let mut bit = [0; 4001];
        let mut lookup = [0; 4001];

        // insert the first n elements in O(n)
        for i in 1..bit.len() as i32 {
            if i <= n {
                lookup[i as usize] = i;
                bit[i as usize] += 1;
            }
            if i + (i & -i) <= bit.len() as i32 {
                bit[(i + (i & -i)) as usize] += bit[i as usize];
            }
        }

        return Self {bit, lookup, size: n};
    }

    fn adjust(&mut self, mut idx: i32, delta: i32) {
        while idx <= 4001 {
            self.bit[idx as usize] += delta;
            idx += idx & -idx;
        }
    }

    fn search(&self, k: i32) -> i32 {
        let mut sum = 0;
        let mut pos = 0;
        
        for i in (0..=self.size.ilog2() as usize).rev() {
            if pos + (1<<i) < self.size as usize && sum + self.bit[pos + (1<<i)] < k {
                sum += self.bit[pos + (1<<i)];
                pos += 1<<i;
            }
        }
        return pos as i32 + 1;
    }

    fn fetch(&mut self, k: i32) -> i32 {
        
        let idx = self.search(k);
        let ret = self.lookup[idx as usize];
        self.adjust(idx, -1);
        self.size += 1;
        self.adjust(self.size, 1);
        self.lookup[self.size as usize] = ret;
        return ret;
    }

}

#[test]
fn tests() {

    let mut m = MRUQueue::new(8);
    println!("{:?}", m.size);
    println!("{:?}", &m.lookup[0..=m.size as usize]);
    println!("{:?}", &m.bit[0..=m.size as usize]);

    let f = m.fetch(3);
    assert_eq!(3, f);
    println!("\n{:?}", &m.lookup[0..=16]);
    println!("{:?}", &m.bit[0..=16]);

    let f = m.fetch(5);
    assert_eq!(6, f);
    println!("\n{:?}", &m.lookup[0..=16]);
    println!("{:?}", &m.bit[0..=16]);

    let f = m.fetch(2);
    assert_eq!(2, f);
    println!("\n{:?}", &m.lookup[0..=16]);
    println!("{:?}", &m.bit[0..=16]);

    let f = m.fetch(8);
    assert_eq!(2, f);
    println!("\n{:?}", &m.lookup[0..=16]);
    println!("{:?}", &m.bit[0..=16]);
}
