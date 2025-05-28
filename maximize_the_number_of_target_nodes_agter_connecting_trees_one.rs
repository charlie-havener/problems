use std::collections::VecDeque;

pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {

    if k == 0 {
        return vec![1; edges1.len() + 1];
    }

    let mut edge_map1: Vec<Vec<usize>> = vec![vec![]; edges1.len() + 1];
    let mut edge_map2: Vec<Vec<usize>> = vec![vec![]; edges2.len() + 1];

    for e in &edges1 {
        let (a,b) = (e[0] as usize, e[1] as usize);
        edge_map1[a].push(b);
        edge_map1[b].push(a);
    }

    for e in &edges2 {
        let (a,b) = (e[0] as usize, e[1] as usize);
        edge_map2[a].push(b);
        edge_map2[b].push(a);
    }

    let mut max_count_2 = 1;
    if k > 1 {
        for m in 0..=edges2.len() {
            max_count_2 =  max_count_2.max(bfs_limited(&edge_map2, m, k-1));
        }
    }

    let mut ans = Vec::with_capacity(edges1.len() + 1);
    for n in 0..=edges1.len() {
        ans.push(max_count_2 + bfs_limited(&edge_map1, n, k));
    }

    return ans;
}



fn bfs_limited(edge_map: &Vec<Vec<usize>>, root: usize, k: i32) -> i32 {
    let mut count = 1;
    let mut visited = vec![false; edge_map.len()];

    let mut queue = VecDeque::from([root]);
    let mut level = 0;
    loop {
        if level >= k || queue.is_empty() {
            return count;
        }

        let c = queue.len();
        for _ in 0..c {
            let t = queue.pop_front().unwrap();
            visited[t] = true;
            for nei in &edge_map[t] {
                if !visited[*nei] {
                    count += 1;
                    queue.push_back(*nei);
                }
            }
        }
        
        level += 1;
    }
}

#[test]
fn tests() {

    let edges1 = vec![vec![0,1],vec![0,2],vec![2,3],vec![2,4]];
    let edges2 = vec![vec![0,1],vec![0,2],vec![0,3],vec![2,7],vec![1,4],vec![4,5],vec![4,6]];
    let k = 2;
    assert_eq!(vec![9,7,9,8,8], max_target_nodes(edges1, edges2, k));

    let edges1 = vec![vec![0,1],vec![0,2],vec![0,3],vec![0,4]];
    let edges2 = vec![vec![0,1],vec![1,2],vec![2,3]];
    let k = 1;
    assert_eq!(vec![6,3,3,3,3], max_target_nodes(edges1, edges2, k));

    let edges1 = vec![vec![0,1],vec![0,2],vec![0,3],vec![0,4]];
    let edges2 = vec![vec![0,1],vec![1,2],vec![2,3]];
    let k = 0;
    assert_eq!(vec![1,1,1,1,1], max_target_nodes(edges1, edges2, k));

}
















