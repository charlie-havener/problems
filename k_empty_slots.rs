struct FenwickTree {
    bit: Vec<i32>,
    on: Vec<bool>,
    size: i32,
}

impl FenwickTree {
    fn new(size: i32) -> Self {
        return Self {
            bit: vec![0; size as usize + 1],
            on: vec![false; size as usize + 1],
            size: size + 1
        };
    }

    fn update(&mut self, mut idx: i32, delta: i32) {
        while idx < self.size {
            self.bit[idx as usize] += delta;
            idx += idx & (-1 * idx);
        }
    }

    fn sum(&self, mut idx: i32) -> i32 {
        let mut sum = 0;
        while idx > 0 {
            sum += self.bit[idx as usize];
            idx -= idx & (-idx);
        }
        return sum;
    }

    fn range_sum(&self, left: i32, right: i32) -> i32 {
        return self.sum(right) - self.sum(left);
    }
}


pub fn k_empty_slots(bulbs: Vec<i32>, k: i32) -> i32 {
    let n = bulbs.len() as i32;
    let mut t = FenwickTree::new(n);
    for (idx,b) in bulbs.into_iter().enumerate() {
        t.update(b, 1);
        t.on[b as usize] = true;

        // check the k range under b
        if b > k + 1 {
            if t.range_sum(b-k-1, b) == 1 && t.on[(b-k-1) as usize] {
                return idx as i32 + 1;
            }
        }

        // check the k range above b
        if b <= n - k - 1 {
            if t.range_sum(b, b+k+1) == 1 && t.on[(b+k+1) as usize] {
                return idx as i32 + 1;
            }
        }
    }

    return -1;
}

