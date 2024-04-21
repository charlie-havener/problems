use std::collections::HashMap;

pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let mut edge_map: HashMap<i32, Vec<i32>> = edges.into_iter().fold(HashMap::new(), |mut acc, e| {
        let (a,b) = (e[0], e[1]);
        acc.entry(a).and_modify(|v| v.push(b)).or_insert(vec![b]);
        acc.entry(b).and_modify(|v| v.push(a)).or_insert(vec![a]);
        acc
    });

    let mut pq = vec![source];
    let mut npq = vec![];
    let mut visited = vec![false; n as usize];

    while pq.len() > 0 {
        while let Some(v) = pq.pop() {
            if v == destination {
                return true;
            }
            if visited[v as usize] {
                continue;
            }
            visited[v as usize] = true;

            for &i in edge_map.get(&v).unwrap_or(&vec![]) {
                npq.push(i);
            }
        }
        std::mem::swap(&mut pq, &mut npq);
    }

    return false;
}

#[test]
fn tests() {
    let n = 3;
    let edges = vec![vec![0,1],vec![1,2],vec![2,0]];
    let source = 0;
    let destination = 2;
    assert_eq!(true, valid_path(n, edges, source, destination));

    let n = 6;
    let edges = vec![vec![0,1],vec![0,2],vec![3,5],vec![5,4],vec![4,3]];
    let source = 0;
    let destination = 5;
    assert_eq!(false, valid_path(n, edges, source, destination));
}
