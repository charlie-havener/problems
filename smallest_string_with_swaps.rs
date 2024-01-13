use std::cmp::Ordering;
use std::collections::HashMap;


struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(x: usize) -> Self {
        Self {
            parent: (0..x as usize).collect(),
            rank: vec![0; x as usize],
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
        if root1 == root2 { return; }

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

fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
    let mut uf = UnionFind::new(s.len());
    let mut bytes: Vec<u8> = s.clone().bytes().collect();
    
    for p in pairs {
        uf.union(p[0] as usize, p[1] as usize);
    }

    let mut hm: HashMap<usize, Vec<usize>> = HashMap::new();
    for idx in 0..s.len(){
        let root = uf.find(idx);
        hm.entry(root)
            .and_modify(|x| x.push(idx))
            .or_insert(vec![idx]);
    }
    
    for (_, value) in &hm {
        let mut chars = vec![];
        for v in value {
            chars.push(bytes[*v]);
        }
        chars.sort();
        for (chr, idx) in chars.iter().zip(value.iter()) {
            bytes[*idx] = *chr;
        }
    }
    
    return String::from_utf8(bytes).unwrap();
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        
        assert_eq!(
            String::from("bacd"),
            smallest_string_with_swaps(String::from("dcab"), vec![vec![0,3],vec![1,2]])
        );

        assert_eq!(
            String::from("abcd"),
            smallest_string_with_swaps(String::from("dcab"),vec![vec![0,3],vec![1,2],vec![0,2]]) 
        );

        assert_eq!(
            String::from("abc"),
            smallest_string_with_swaps(String::from("cba"), vec![vec![0,1],vec![1,2]])
        );
    }
}
