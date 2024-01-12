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
        if root1 == root2 { return } // no union to be done...

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


fn earliest_acq(mut logs: Vec<Vec<i32>>, n: i32) -> i32 {
    logs.sort_by(|x, y| x[0].cmp(&y[0]));

    let mut uf = UnionFind::new(n);
    for e in logs {
        uf.union(e[1] as usize, e[2] as usize);
        if uf.count == 1 { return e[0] }
    }
    return -1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut logs: Vec<Vec<i32>>;

        logs = vec![vec![20190101,0,1],vec![20190104,3,4],vec![20190107,2,3],vec![20190211,1,5],vec![20190224,2,4],vec![20190301,0,3],vec![20190312,1,2],vec![20190322,4,5]];
        assert_eq!(20190301, earliest_acq(logs, 6));

        logs = vec![vec![0,2,0],vec![1,0,1],vec![3,0,3],vec![4,1,2],vec![7,3,1]];
        assert_eq!(3, earliest_acq(logs, 4));

        logs = vec![vec![1,0,1],vec![0,2,3]];
        assert_eq!(-1, earliest_acq(logs, 4));

        logs = vec![vec![203,0,3],vec![200,2,0],vec![201,0,1],vec![204,1,2],vec![207,3,1]];
        assert_eq!(203, earliest_acq(logs, 4));
    }
}
