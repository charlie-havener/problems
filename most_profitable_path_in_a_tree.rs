use std::collections::{HashMap, VecDeque};

pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {

    let mut edges: HashMap<i32, Vec<i32>> = edges.iter().fold(HashMap::new(), |mut acc, edg| {
        let (a,b) = (edg[0], edg[1]);
        acc.entry(a).and_modify(|v| v.push(b)).or_insert(vec![b]);
        acc.entry(b).and_modify(|v| v.push(a)).or_insert(vec![a]);
        acc
    });

    let mut ans = 0;

    let mut visited = vec![vec![-1; edges.len() + 1]];

    // (person, node, score) 0=>Alice,1=>Bob
    let mut queue = VecDeque::new();
    queue.push_back((0,0,0));
    queue.push_back((1,bob,0));

    let mut time = 0;
    while !queue.is_empty() {
        for _ in 0..queue.len() {
            let (person, node, mut score) = queue.pop_back().unwrap();
            score += amount[node as usize];
            visited[person as usize][node as usize] = time;
            if visited[((person + 1) % 2) as usize][node as usize] == time {
                
            }
        }
    }




    return ans;
}
