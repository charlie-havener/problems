use std::collections::VecDeque;

pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {

    let n = n as usize;

    // edges -> adj list
    let mut graph = vec![vec![]; n];
    for e in edges {
        let a = e[0] as usize - 1;
        let b = e[1] as usize - 1;
        graph[a].push(b);
        graph[b].push(a);
    }

    // bipartite check
    let mut group = vec![-1; n];
    let mut queue = VecDeque::new();
    for node in 0..n {
        if group[node] != -1 {
            continue;
        }
        group[node] = 0;
        queue.push_back(0);
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let p = queue.pop_front().unwrap();
                for nei in &graph[p] {
                    if group[*nei] == group[p] {
                        return -1;
                    }
                    if group[*nei] == -1 {
                        group[*nei] = (group[p] * -1) + 1; // 0 -> 1; 1 -> 0
                        queue.push_back(*nei);
                    }
                }
            }
        }
    }
    // if it makes it here then it is bipartite, o.w. returns -1 in loop above

    // find the node of each group that starts the longest path
    let mut dists = vec![0; n];
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    for node in 0..n {
        visited.fill(false);
        visited[node] = true;
        queue.push_back(node);
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let p = queue.pop_front().unwrap();
                for nei in &graph[p] {
                    if visited[*nei] { continue }
                    visited[*nei] = true;
                    queue.push_back(*nei)
                }
            }
            dists[node] += 1;
        }
    }

    // get the largest dist of each component and sum them
    let mut visited = vec![false; n];
    let mut ans = 0;
    let mut queue = VecDeque::new();
    for node in 0..n {
        if visited[node] { continue }
        let mut component = dists[node];
        queue.push_back(node);
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let p = queue.pop_front().unwrap();
                component = component.max(dists[p]);
                for nei in &graph[p] {
                    if visited[*nei] { continue }
                    visited[*nei] = true;
                    queue.push_back(*nei);
                }
            }
        }
        ans += component;

    }

    return ans;
}

#[test]
fn tests() {
    //let n = 6;
    //let edges = vec![vec![1,2],vec![1,4],vec![1,5],vec![2,6],vec![2,3],vec![4,6]];
    //assert_eq!(4, magnificent_sets(n, edges));

    //let n = 3;
    //let edges = vec![vec![1,2],vec![2,3],vec![3,1]];
    //assert_eq!(-1, magnificent_sets(n, edges));

    let n = 3;
    let edges = vec![vec![1,3]];
    assert_eq!(3, magnificent_sets(n, edges));
}
