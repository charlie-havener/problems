use std::collections::{HashMap, HashSet};

pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut eg: HashMap<i32, HashSet<i32>> = edges.iter().fold(HashMap::new(), |mut acc, e| {
        acc.entry(e[0]).and_modify(|s| {s.insert(e[1]);}).or_insert(HashSet::from([e[1]]));
        acc.entry(e[1]).and_modify(|s| {s.insert(e[0]);}).or_insert(HashSet::from([e[0]]));
        acc
    });
    let mut nqueue = Vec::new();
    let mut queue = Vec::new();
    for (&k,v) in eg.iter() {
        if v.len() == 1 {
            queue.push(k);
        }
    }

    let mut remaining_edges = n;
    while remaining_edges > 2 {
        while let Some(n) = queue.pop() {
            remaining_edges -= 1;
            let (k, v) = eg.remove_entry(&n).unwrap();
            let v = v.into_iter().next().unwrap();
            eg.get_mut(&v).unwrap().remove(&k);
            if eg.get(&v).unwrap().len() == 1 {
                nqueue.push(v);
            }
        }
        std::mem::swap(&mut queue, &mut nqueue);
    }
    return queue;
}

#[test]
fn tests() {
    let n = 4;
    let edges = vec![vec![1,0],vec![1,2],vec![1,3]];
    assert_eq!(vec![1], find_min_height_trees(n, edges));

    let n = 6;
    let edges = vec![vec![3,0],vec![3,1],vec![3,2],vec![3,4],vec![5,4]];
    assert_eq!(vec![4,3], find_min_height_trees(n, edges));
}
