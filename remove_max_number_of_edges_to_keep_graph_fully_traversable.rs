struct UnionFind {
    alice_parents: Vec<usize>,
    alice_ranks: Vec<usize>,
    alice_count: usize,

    bob_parents: Vec<usize>,
    bob_ranks: Vec<usize>,
    bob_count: usize,

    edge_count: i32,
}

impl UnionFind {
    fn new(n: i32) -> Self {
        let n = n as usize;
        Self {
            alice_parents: (0..n).collect::<Vec<_>>(),
            alice_ranks: vec![0; n],
            alice_count: n,

            bob_parents: (0..n).collect::<Vec<_>>(),
            bob_ranks: vec![0; n],
            bob_count: n,

            edge_count: 0,
        }
    }

    fn find(&mut self, p: &Person, n: usize) -> usize {
        match p {
            Person::Alice => {
                if self.alice_parents[n] != n {
                    self.alice_parents[n] = self.find(p, self.alice_parents[n]);
                }
                return self.alice_parents[n];
            }
            Person::Bob => {
                if self.bob_parents[n] != n {
                    self.bob_parents[n] = self.find(p, self.bob_parents[n]);
                }
                return self.bob_parents[n];
            }
        };
    }

    fn union(&mut self, p: &Person, x: usize, y: usize) -> () {
        let p1 = self.find(&p, x);
        let p2 = self.find(&p, y);
        if p1 == p2 { return }

        let (parents, ranks, count) = match p {
            Person::Alice => (&mut self.alice_parents, &mut self.alice_ranks, &mut self.alice_count),
            Person::Bob => (&mut self.bob_parents, &mut self.bob_ranks, &mut self.bob_count),
        };
        
        match ranks[p1].cmp(&ranks[p2]) {
            std::cmp::Ordering::Less => parents[p1] = p2,
            std::cmp::Ordering::Greater => parents[p2] = p1,
            _ => {
                parents[p1] = p2;
                ranks[p2] += 1;
            },
        }
        *count -= 1;
        self.edge_count += 1;
    }
}

enum Person {
    Alice,
    Bob,
}

pub fn max_num_edges_to_remove(n: i32, mut edges: Vec<Vec<i32>>) -> i32 {
    
    // prioritizes the 3s
    edges.sort_by(|a,b| b.cmp(a));

    let mut uf = UnionFind::new(n);


    for edge in &edges {
        let p = edge[0];
        let x = edge[1] - 1;
        let y = edge[2] - 1;

        match p {
            3 => {
                let s = uf.edge_count;
                uf.union(&Person::Alice, x as usize, y as usize);
                uf.union(&Person::Bob, x as usize, y as usize);
                if uf.edge_count == s + 2 {
                    uf.edge_count -= 1;
                }
            },
            2 => {
                uf.union(&Person::Bob, x as usize, y as usize);
            },
            1 => {
                uf.union(&Person::Alice, x as usize, y as usize);
            },
            _ => unreachable!(),
        }
    }




    if uf.alice_count != 1 || uf.bob_count != 1 {
        return -1;
    }
    return edges.len() as i32 - uf.edge_count;
}

#[test]
fn tests() {
    let n = 4;
    let e = vec![vec![3,1,2],vec![3,2,3],vec![1,1,3],vec![1,2,4],vec![1,1,2],vec![2,3,4]];
    assert_eq!(2, max_num_edges_to_remove(n,e));

    let n = 4;
    let e = vec![vec![3,1,2],vec![3,2,3],vec![1,1,4],vec![2,1,4]];
    assert_eq!(0, max_num_edges_to_remove(n,e));

    let n = 4;
    let e = vec![vec![3,2,3],vec![1,1,2],vec![2,3,4]];
    assert_eq!(-1, max_num_edges_to_remove(n,e));
}
