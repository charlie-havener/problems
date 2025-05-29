pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {

    let mut edge_map1: Vec<Vec<usize>> = vec![vec![]; edges1.len() + 1];
    for e in edges1 {
        let (a,b) = (e[0] as usize, e[1] as usize);
        edge_map1[a].push(b);
        edge_map1[b].push(a);
    }

    let mut edge_map2: Vec<Vec<usize>> = vec![vec![]; edges2.len() + 1];
    for e in edges2 {
        let (a,b) = (e[0] as usize, e[1] as usize);
        edge_map2[a].push(b);
        edge_map2[b].push(a);
    }

    // determine maxcount in graph2
    let mut counts = [0,0];
    let mut groups2 = vec![2; edge_map2.len()];
    dfs(&edge_map2, 0, &mut groups2, 0);

    let ones_count2 = groups2.iter().filter(|v| **v == 1).count() as i32;
    let max_count2 = ones_count2.max(edge_map2.len() as i32 - ones_count2);


    let mut groups1 = vec![2; edge_map1.len()];
    dfs(&edge_map1, 0, &mut groups1, 0);

    let ones_count1 = groups1.iter().filter(|v| **v == 1).count() as i32;
    let zero_count1 = groups1.len() as i32 - ones_count1;
    for ele in groups1.iter_mut() {
        if *ele == 1 {
            *ele = ones_count1 + max_count2;
        }
        else {
            *ele = zero_count1 + max_count2;
        }
    }

    return groups1; 
}


fn dfs(edge_map: &Vec<Vec<usize>>, node: usize, visited: &mut Vec<i32>, parity: i32) -> () {
    if visited[node] != 2 { return }
    visited[node] = parity;
    for nei in &edge_map[node] {
        if visited[*nei] == 2 {
            dfs(edge_map, *nei, visited, parity^1);
        }
    }
}
