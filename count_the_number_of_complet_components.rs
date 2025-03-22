use std::cmp::Ordering;

struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
    edges: Vec<usize>,
    nodes: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect::<Vec<usize>>(),
            ranks: vec![0; n],
            edges: vec![0; n],
            nodes: vec![1; n],
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
        if px == py {
            self.edges[px] += 1;
            return;
        }
        match self.ranks[px].cmp(&self.ranks[py]) {
            Ordering::Less => {
                self.parents[px] = py;
                self.edges[py] += self.edges[px] + 1;
                self.nodes[py] += self.nodes[px];
            },
            Ordering::Greater => {
                self.parents[py] = px;
                self.edges[px] += self.edges[py] + 1;
                self.nodes[px] += self.nodes[py];
            },
            Ordering::Equal => {
                self.parents[py] = px;
                self.edges[px] += self.edges[py] + 1;
                self.nodes[px] += self.nodes[py];
                self.ranks[px] += 1;
            },
        }
    }

    fn complete_components(&self) -> i32 {
        let mut ans = 0;
        for idx in 0..self.parents.len() {
            if idx == self.parents[idx] {
                let e_count = self.edges[idx];
                let n_count = self.nodes[idx];
                if n_count * (n_count-1) / 2 == e_count {
                    ans += 1;
                }
            }
        }
        return ans;
    }
}


pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    
    let mut uf = UnionFind::new(n as usize);
    for e in edges {
        uf.union(e[0] as usize, e[1] as usize);
    }
    return uf.complete_components();
}

#[test]
fn tests() {
    let n = 6;
    let edges = vec![vec![0,1],vec![0,2],vec![1,2],vec![3,4]];
    assert_eq!(3, count_complete_components(n, edges));

    let n = 6;
    let edges = vec![vec![0,1],vec![0,2],vec![1,2],vec![3,4],vec![3,5]];
    assert_eq!(1, count_complete_components(n, edges));
}
