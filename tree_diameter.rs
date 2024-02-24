use std::collections::HashMap;


pub fn dfs(node: &i32, hm: &HashMap<i32, Vec<i32>>, visited: &mut Vec<i32>) -> (i32,i32) {
    let mut ans = 0;
    let mut furthest_node = *node;
    for &child in hm.get(node).unwrap() {
        if visited[child as usize] == 1 { continue; }
        visited[child as usize] = 1;
        let (depth, v) = dfs(&child, hm, visited);
        if ans < depth + 1 {
            ans = depth + 1;
            furthest_node = v;
        }
    }
    return (ans, furthest_node);
}


pub fn tree_diameter(edges: Vec<Vec<i32>>) -> i32 {
    let n = edges.len() + 1;
    if n == 1 { return 0 }
    if n == 2 { return 1 }
    let hm = edges.iter().fold(HashMap::new(), |mut acc: HashMap<i32, Vec<_>>, e| {
        acc.entry(e[0]).and_modify(|v| v.push(e[1])).or_insert(vec![e[1]]);
        acc.entry(e[1]).and_modify(|v| v.push(e[0])).or_insert(vec![e[0]]);
        acc
    });
    let mut visited = vec![0;n];
    let (_, furthest) = dfs(&0, &hm, &mut visited);
    drop(visited);
    let mut visited = vec![0;n];
    let (ans, _) = dfs(&furthest, &hm, &mut visited);
    return ans;
}

#[cfg(test)]
mod test {
    use super::tree_diameter;

    #[test]
    fn test() {
        //let edges = vec![vec![0,1],vec![0,2]];
        //assert_eq!(2, tree_diameter(edges));
        let edges = vec![vec![0,1],vec![1,2],vec![2,3],vec![1,4],vec![4,5]];
        assert_eq!(4, tree_diameter(edges));
        let edges = vec![];
        assert_eq!(0, tree_diameter(edges));
    }
}
