use std::cmp::Ordering;

struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
    costs: Vec<i32>,
}

impl UnionFind {

    fn new(n: i32) -> Self {
        Self {
            parents: (0..n as usize).collect(),
            ranks: vec![0; n as usize],
            costs: vec![-1; n as usize],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x != self.parents[x] {
            self.parents[x] = self.find(self.parents[x]);
        }
        return self.parents[x];
    }

    fn union(&mut self, x: usize, y: usize) -> () {
        let px = self.find(x);
        let py = self.find(y);
        if px == py { return }
        match self.ranks[px].cmp(&self.ranks[py]) {
            Ordering::Less => self.parents[px] = py,
            Ordering::Greater => self.parents[py] = px,
            Ordering::Equal => {
                self.parents[py] = px;
                self.ranks[px] += 1;
            }
        }
    }

    fn are_connected(&mut self, x: usize, y: usize) -> bool {
        return self.find(x) == self.find(y);
    }

}



pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {

    let mut ans = Vec::with_capacity(query.len());
    
    let mut uf = UnionFind::new(n);
    for edge in &edges {
        uf.union(edge[0] as usize, edge[1] as usize);
    }
    for edge in &edges {
        let r = uf.find(edge[0] as usize);
        uf.costs[r] &= edge[2];
    }

    for q in &query {
        if !uf.are_connected(q[0] as usize, q[1] as usize) {
            ans.push(-1);
        } else {
            let r = uf.find(q[0] as usize);
            ans.push(uf.costs[r]);
        }
    }

    return ans;
}
