struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            ranks: vec![0;n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parents[x] != x {
            self.parents[x] = self.find(self.parents[x]);
        }
        self.parents[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root1 = self.find(x);
        let root2 = self.find(y);
        if root1 == root2 { return }
        match self.ranks[root1].cmp(&self.ranks[root2]) {
            std::cmp::Ordering::Less => self.parents[root1] = root2,
            std::cmp::Ordering::Greater => self.parents[root2] = root1,
            _ => {
                self.parents[root2] = root1;
                self.ranks[root1] += 1;
            }
        }
    }
}


pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
    let max_num = 100_000;
    if nums.len() == 1 { return true }
    
    let mut primes = vec![0; max_num+1];
    for idx in 2..primes.len() {
        let mut v = idx;
        if primes[idx] == 0 {
            while v <= max_num {
                primes[v] = idx;
                v+= idx;
            }
        }
    }

    let mut uf = UnionFind::new(max_num + 1);

    for idx in 0..nums.len() {
        let mut n = nums[idx] as usize;
        if n == 1 { return false }
        let mut p_factors = vec![];
        while n > 1 {
            p_factors.push(primes[n]);
            n /= primes[n];
        }

        for f in p_factors {
            uf.union(nums[idx] as usize, f);
        }
    }

    let root = uf.find(nums[0] as usize);
    for idx in 1..nums.len() {
        if uf.find(nums[idx] as usize) != root {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let v = vec![2,3,6];
        assert_eq!(true, can_traverse_all_pairs(v));

        let v = vec![3,9,5];
        assert_eq!(false, can_traverse_all_pairs(v));

        let v = vec![4,3,12,8];
        assert_eq!(true, can_traverse_all_pairs(v));

        let v = vec![1,1];
        assert_eq!(false, can_traverse_all_pairs(v));

        let v = vec![1];
        assert_eq!(true, can_traverse_all_pairs(v));
        
        let v = vec![100_000, 99_999, 99_998];
        assert_eq!(true, can_traverse_all_pairs(v));
    }
}
