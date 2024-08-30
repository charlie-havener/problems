use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

pub fn modified_graph_edges(n: i32, mut edges: Vec<Vec<i32>>, source: i32, destination: i32, target: i32) -> Vec<Vec<i32>> {
    println!("---------------");
    let source = source as usize;
    let dest = destination as usize;
    let target = target as usize;
    let edge_count = edges.len();

    // construct an edge map only using the non negative weight edges
    let mut edge_map: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    for e in &edges {
        if e[2] == -1 { continue }

        let (a, b, w) = (e[0] as usize, e[1] as usize, e[2] as usize);
        edge_map.entry(a).and_modify(|v| v.push((b,w))).or_insert(vec![(b,w)]);
        edge_map.entry(b).and_modify(|v| v.push((a,w))).or_insert(vec![(a,w)]);
    }


    // find shortest path from source to dest ignoring the -1 weight edges
    // if a path exists that is less than target, then return an empty vec
    let path_sum = dijkstra(&edge_map, source, dest, n);
    println!("initial: {path_sum}");

    if path_sum < target { return vec![] }
    if path_sum == target {
        for e in edges.iter_mut() {
            if e[2] == -1 { e[2] = 2_000_000_000 }
        }
        return edges;
    }


    // now add in the negative weight edges one at a time and rerun dijkstra
    for edge_idx in 0..edge_count {

        if edges[edge_idx][2] != -1 { continue }
        edges[edge_idx][2] = 1;

        let (a,b,w) = (edges[edge_idx][0] as usize, edges[edge_idx][1] as usize, edges[edge_idx][2] as usize);
        edge_map.entry(a).and_modify(|v| v.push((b,w))).or_insert(vec![(b,w)]);
        edge_map.entry(b).and_modify(|v| v.push((a,w))).or_insert(vec![(a,w)]);

        let path_sum = dijkstra(&edge_map, source, dest, n);
        println!("{edge_idx}: {path_sum}");

        if path_sum <= target {
            edges[edge_idx][2] += (target - path_sum) as i32;

            for e in edges.iter_mut() {
                if e[2] == -1 { e[2] = 2_000_000_000 }
            }
            return edges;
        }
    }
    return vec![];
}


fn dijkstra(edge_map: &HashMap<usize, Vec<(usize, usize)>>, source: usize, dest: usize, node_count: i32) -> usize {

    // pq holds (Reverse(path_sum), node) pairs
    let mut pq: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
    pq.push((Reverse(0), source));
    let mut visited = vec![false; node_count as usize];
    while !pq.is_empty() {
        let (Reverse(path_sum), curr) = pq.pop().unwrap();
        
        if visited[curr] { continue }
        visited[curr] = true;

        if curr == dest {
            return path_sum;
        }

        for neighbor in edge_map.get(&curr).unwrap_or(&Vec::new()) {
            let (nei, weight) = *neighbor;
            if !visited[nei] {
                pq.push((Reverse(path_sum + weight), nei));
            }
        }
    }

    return 2_000_000_000;
}



#[test]
fn tests() {
    //let n = 5;
    //let edges = vec![vec![4,1,-1],vec![2,0,-1],vec![0,3,-1],vec![4,3,-1]];
    //let source = 0;
    //let destination = 1;
    //let target = 5;
    //println!("{:?}", modified_graph_edges(n, edges, source, destination, target));

    //let n = 3;
    //let edges = vec![vec![0,1,-1],vec![0,2,5]];
    //let source = 0;
    //let destination = 2;
    //let target = 6;
    //println!("{:?}", modified_graph_edges(n, edges, source, destination, target));

    //let n = 4;
    //let edges = vec![vec![1,0,4],vec![1,2,3],vec![2,3,5],vec![0,3,-1]];
    //let source = 0;
    //let destination = 2;
    //let target = 6;
    //println!("{:?}", modified_graph_edges(n, edges, source, destination, target));

    //let n = 3;
    //let edges = vec![vec![0,1,-1],vec![1,2,-1],vec![0,2,6]];
    //let source = 0;
    //let destination = 2;
    //let target = 9;
    //println!("{:?}", modified_graph_edges(n, edges, source, destination, target));

    //let n = 3;
    //let edges = vec![vec![0,1,4],vec![1,2,4],vec![0,2,-1]];
    //let source = 0;
    //let destination = 2;
    //let target = 9;
    //println!("{:?}", modified_graph_edges(n, edges, source, destination, target));

    let n = 5;
    let edges = vec![vec![1,4,1],vec![2,4,-1],vec![3,0,2],vec![0,4,-1],vec![1,3,10],vec![1,0,10]];
    let source = 0;
    let destination = 2;
    let target = 15;
    println!("{:?}", modified_graph_edges(n, edges, source, destination, target));
}
