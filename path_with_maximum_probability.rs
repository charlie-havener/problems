use std::collections::BinaryHeap;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
struct Custf64 (f64);
impl Eq for Custf64 {}
impl Ord for Custf64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}


pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start_node: i32, end_node: i32) -> f64 {

    let mut egs = vec![Vec::new(); n as usize];
    for i in 0..edges.len() {
        let a = edges[i][0] as usize;
        let b = edges[i][1] as usize;
        let p = Custf64(succ_prob[i]);

        egs[a].push((b, p));
        egs[b].push((a, p));
    }

    // (node, prob so far)
    let mut pq = BinaryHeap::new();
    pq.push((Custf64(1.0), start_node as usize));

    while !pq.is_empty() {
        let (prob, curr) = pq.pop().unwrap();
        if curr == end_node as usize { return prob.0 }

        for (nei, succ) in egs[curr].iter() {
            pq.push((Custf64(prob.0 * succ.0), *nei));
        }
        
        egs[curr].clear();
    }
    


    return 0.0;
}

#[test]
fn tests() {
    let n = 3;
    let e = vec![vec![0,1],vec![1,2],vec![0,2]];
    let s = vec![0.5,0.5,0.2];
    let a = 0;
    let b = 2;
    assert_eq!(0.25, max_probability(n, e, s, a, b));

    let n = 3;
    let e = vec![vec![0,1],vec![1,2],vec![0,2]];
    let s = vec![0.5,0.5,0.3];
    let a = 0;
    let b = 2;
    assert_eq!(0.3, max_probability(n, e, s, a, b));

    let n = 3;
    let e = vec![vec![0,1]];
    let s = vec![0.5];
    let a = 0;
    let b = 2;
    assert_eq!(0.0, max_probability(n, e, s, a, b));
}
