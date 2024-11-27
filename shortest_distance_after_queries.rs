use std::collections::{HashMap, VecDeque};

pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    
    let mut ans = Vec::with_capacity(queries.len());

    let mut edges: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..(n as usize - 1) {
        edges.insert(i, vec![i+1]);
    }

    //println!("{edges:?}");

    for q in queries {
        // add the new edge
        let (from, to) = (q[0] as usize, q[1] as usize);
        edges.entry(from).and_modify(|t| t.push(to));

        //bfs
        let mut visited = vec![false; n as usize];
        visited[0] = true;
        let mut pq: VecDeque<(usize, i32)> = VecDeque::from([(0,0)]); // (node, steps)

        while let Some((node, step)) = pq.pop_front() {
            if node == n as usize - 1 { 
                ans.push(step);
                break;
            }
            for nei in edges.get(&node).unwrap() {
                if visited[*nei] { continue }
                visited[*nei] = true;
                pq.push_back((*nei, step+1));
            }
        }
    }
    return ans;
}
