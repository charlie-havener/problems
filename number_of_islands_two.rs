use std::cmp::Ordering;

struct UnionFind {
    parents: Vec<Option<usize>>,
    ranks: Vec<usize>,
    count: i32,
}

impl UnionFind {
    fn new(v: i32) -> Self {
        Self {
            parents: vec![None; v as usize],
            ranks: vec![0; v as usize],
            count: 0,
        }
    }
    fn is_land(&self, v: usize) -> bool {
        self.parents[v].is_some()
    }

    fn add_land(&mut self, v: usize) {
        if self.parents[v] == None {
            self.parents[v] = Some(v);
            self.count += 1;
        }
    }

    fn find(&mut self, v: usize) -> Option<usize> {
        if self.parents[v] != Some(v) {
            self.parents[v] = self.find(self.parents[v]?);
        }
        return self.parents[v];
    }

    fn union(&mut self, i: usize, j: usize) {
        let r1 = self.find(i).unwrap();
        let r2 = self.find(j).unwrap();
        if r1 == r2 {
            return; // already in the same group
        }
        match self.ranks[r1].cmp(&self.ranks[r2]) {
            Ordering::Less => self.parents[r1] = self.parents[r2],
            Ordering::Greater => self.parents[r2] = self.parents[r1],
            Ordering::Equal => {
                self.parents[r2] = self.parents[r1];
                self.ranks[r1] += 1;
            }
        }
        self.count -= 1;
    }
}


pub fn num_islands2(m: i32, n: i32, positions: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans = Vec::with_capacity(positions.len());
    let mut uf = UnionFind::new(m*n);

    let x = vec![-1,1,0,0];
    let y = vec![0,0,-1,1];


    for p in positions {
        let idx = (p[0] * n + p[1]) as usize;
        if uf.is_land(idx) {
            ans.push(uf.count);
            continue;
        }
        uf.add_land(idx);

        for i in 0..x.len() {
            let ne_x = p[0] + x[i];
            let ne_y = p[1] + y[i];
            let ne = ne_x * n + ne_y;
            println!("{ne}");
            if ne_x >= 0 && ne_y >= 0 && ne_x < m && ne_y < n && uf.is_land(ne as usize) {
                uf.union(idx, ne as usize);
            }
        }
        ans.push(uf.count);
    }
    return ans;
}


#[test]
fn tests() {
    let (m,n) = (3,3);
    let pos = vec![vec![0,0],vec![0,1],vec![1,2],vec![2,1]];
    assert_eq!(vec![1,1,2,3], num_islands2(m,n,pos));

    let (m,n) = (1,1);
    let pos = vec![vec![0,0]];
    assert_eq!(vec![1], num_islands2(m,n,pos));
}
