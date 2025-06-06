struct UnionFind {
    parents: [usize; 26],
}

impl UnionFind {
    fn new() -> Self {
        let mut p = [0; 26];
        for i in 0..26 {
            p[i] = i;
        }
        return Self { parents: p }
    }

    fn find(&mut self, x: usize) -> usize {
        if x != self.parents[x] {
            self.parents[x] = self.find(self.parents[x]);
        }
        return self.parents[x];
    }

    fn union(&mut self, x: usize, y: usize) -> () {
        let p_x = self.find(x);
        let p_y = self.find(y);
        if p_x == p_y { return }
        match p_x.cmp(&p_y) {
            std::cmp::Ordering::Less => self.parents[p_y] = p_x,
            std::cmp::Ordering::Greater => self.parents[p_x] = p_y,
            _ => panic!("how?"),
        }
    }
}


pub fn smallest_equivalent_string(s1: String, s2: String, mut base_str: String) -> String {
    let mut uf = UnionFind::new();
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    for i in 0..s1.len() {
        let a = (s1[i] - b'a') as usize;
        let b = (s2[i] - b'a') as usize;
        uf.union(a,b);
    }
    
    //Safety: all characters are [a-z] and being replaced by an [a-z]
    unsafe {
        for c in base_str.as_bytes_mut() {
            *c = uf.find((*c - b'a') as usize) as u8 + b'a';
        }
    }

    return base_str;
}
