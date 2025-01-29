use std::cmp::Ordering;

struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
}

impl UnionFind {

    fn new(size: usize) -> Self {
        Self {
            parents: (0..size).collect::<Vec<usize>>(),
            ranks: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parents[x] == x { return x }
        self.parents[x] = self.find(self.parents[x]);
        return self.parents[x];
    }

    fn union(&mut self, x: usize, y: usize) -> () {
        let px = self.find(x);
        let py = self.find(y);
        if px == py { return } // already in the same group
        match self.ranks[px].cmp(&self.ranks[py]) {
            Ordering::Less => self.parents[px] = py,
            Ordering::Greater => self.parents[py] = px,
            Ordering::Equal => {
                self.parents[py] = px;
                self.ranks[px] += 1;
            },
        }
    }

    fn are_connected(&self, x: usize, y: usize) -> bool {
        let px = self.find(x);
        let py = self.find(y);
        return px == py;
    }
}


pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {

    let mut uf = UnionFind::new(edges.len());
    for e in edges {
        let x = e[0] as usize - 1;
        let y = e[1] as usize - 1;
        if uf.are_connected(x,y) { return e }
        uf.union(x,y);
    }
    unreachable!();
}
