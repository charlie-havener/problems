use std::cmp::Ordering;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: i32,
}

impl UnionFind {
    fn new(n: i32) -> Self {
        Self {
            parent: (0..n as usize).collect(),
            rank: vec![0; n as usize],
            count: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        return self.parent[x];
    }

    fn union(&mut self, x: usize, y: usize) {
        let root1 = self.find(x);
        let root2 = self.find(y);
        if root1 == root2 { return }

        self.count -= 1;
        match self.rank[root1].cmp(&self.rank[root2]) {
            Ordering::Less => self.parent[root1] = root2,
            Ordering::Greater => self.parent[root2] = root1,
            Ordering::Equal => {
                self.parent[root2] = root1;
                self.rank[root1] += 1;
            }
        }
    }
}


pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut uf = UnionFind::new(n);
    for e in edges {
        uf.union(e[0] as usize, e[1] as usize);
    }
    return uf.count;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, count_components(5, vec![vec![0,1],vec![1,2],vec![3,4]]));
        assert_eq!(1, count_components(5, vec![vec![0,1],vec![1,2],vec![2,3], vec![3,4]]));
        assert_eq!(5, count_components(5, vec![]));
        assert_eq!(2, count_components(4, vec![vec![0,1], vec![2,3]]));
    }
}
