use std::cmp::Ordering;


struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
    count: i32,
}

impl UnionFind {
    fn new(n: i32) -> Self {        
        Self {
            parents: (0..n as usize).collect::<Vec<_>>(),
            ranks: vec![0 as usize; n as usize],
            count: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parents[x] != x {
            self.parents[x] = self.find(self.parents[x]);
        }
        return self.parents[x];
    }

    fn union(&mut self, x: usize, y: usize) {
        let root1 = self.find(x);
        let root2 = self.find(y);
        if root1 == root2 { return }
        self.count -= 1;
        match self.ranks[root1].cmp(&self.ranks[root2]) {
            Ordering::Less => self.parents[root1] = root2,
            Ordering::Greater => self.parents[root2] = root1,
            Ordering::Equal => {
                self.ranks[root1] += 1;
                self.parents[root2] = root1;
            }
        }
    }
}



pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
    if edges.len() != n as usize - 1 { return false; }
    
    let mut uf = UnionFind::new(n);
    for e in edges {
        uf.union(e[0] as usize, e[1] as usize); 
    }
    return uf.count == 1;
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(true, valid_tree(5, vec![vec![0,1],vec![0,2],vec![0,3],vec![1,4]]));
        assert_eq!(false, valid_tree(5, vec![vec![0,1],vec![1,2],vec![2,3],vec![1,3],vec![1,4]])); 
        assert_eq!(true, valid_tree(5, vec![vec![0,1],vec![0,2],vec![0,3],vec![0,4]]));

    }
}
