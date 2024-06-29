struct Topo {
    edges: Vec<Vec<usize>>,
    indegree: Vec<usize>,
}

impl Topo {
    fn new(n: i32) -> Self {
        Self {
            edges: vec![vec![]; n as usize],
            indegree: vec![0; n as usize],
        }
    }

    fn from(n: i32, edges: &Vec<Vec<i32>>) -> Self {
        let mut tr = Self {
            edges: vec![vec![]; n as usize],
            indegree: vec![0; n as usize],
        };
        for e in edges {
            let from = e[0] as usize;
            let to = e[1] as usize;
            tr.edges[from].push(to);
            tr.indegree[to] += 1;
        }
        return tr;
    }

    fn solve(&mut self) -> Vec<Vec<i32>> {
        let mut pq = Vec::new();
        for (idx, &v) in self.indegree.iter().enumerate() {
            if v == 0 { pq.push(idx) }
        }
        let mut torder = Vec::new();
        while !pq.is_empty() {
            let mut npq = Vec::new();
            while let Some(curr) = pq.pop() {
                torder.push(curr);
                for &nei in &self.edges[curr] {
                    self.indegree[nei] -= 1;
                    if self.indegree[nei] == 0 {
                        npq.push(nei);
                    }
                }
            }
            std::mem::swap(&mut pq, &mut npq);
        }

        let mut ancestors = vec![vec![]; self.indegree.len()];
        for node in torder {
            for nei in &self.edges[node] {
                ancestors[*nei].push(node as i32);
                let others = ancestors[node].clone();
                ancestors[*nei].extend(others);
            }
        }

        for an in ancestors.iter_mut() {
            an.sort();
            an.dedup();
        }

        return ancestors;
    }
}


pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    Topo::from(n, &edges).solve()
}

#[test]
fn tests() {
    let n = 8;
    let e = vec![vec![0,3],vec![0,4],vec![1,3],vec![2,4],vec![2,7],vec![3,5],vec![3,6],vec![3,7],vec![4,6]];
    assert_eq!(
        vec![vec![],vec![],vec![],vec![0,1],vec![0,2],vec![0,1,3],vec![0,1,2,3,4],vec![0,1,2,3]],
        get_ancestors(n,e)
    );

    let n = 5;
    let e = vec![vec![0,1],vec![0,2],vec![0,3],vec![0,4],vec![1,2],vec![1,3],vec![1,4],vec![2,3],vec![2,4],vec![3,4]];
    assert_eq!(
        vec![vec![],vec![0],vec![0,1],vec![0,1,2],vec![0,1,2,3]],
        get_ancestors(n,e)
    );
}
