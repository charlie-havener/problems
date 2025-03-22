use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn minimum_distance(n: i32, edges: Vec<Vec<i32>>, s: i32, marked: Vec<i32>) -> i32 {

    // visited array. 0 => unvisited, 1 => visited, 2 => marked
    // if node is visited then continue,
    // if node is unvisited then traverse,
    // if node is marked then return
    let mut visited = vec![0; n as usize];
    for m in marked {
        visited[m as usize] = 2;
    }

    // from: (to, weight)
    let mut graph = vec![vec![]; n as usize];
    for edge in edges {
        graph[edge[0] as usize].push((edge[1], edge[2]));
    }

    let mut pq: BinaryHeap<(Reverse<i32>, i32)> = BinaryHeap::new();
    pq.push((Reverse(0), s));
    while !pq.is_empty() {
        let (Reverse(weight), curr) = pq.pop().unwrap();
        match visited[curr as usize] {
            1 => continue,
            2 => return weight,
            0 => {
                for (nei, w) in &graph[curr as usize] {
                    if visited[*nei as usize] != 1 {
                        pq.push((Reverse(weight + *w), *nei));
                    }
                }
            },
            _ => unreachable!(),
        }
        visited[curr as usize] = 1;
    }
    
    return -1;
}
