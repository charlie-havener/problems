use std::collections::{HashMap, VecDeque, HashSet};

pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, mut values: Vec<i32>, k: i32) -> i32 {

    if n == 1 { return 1 }


    let mut ans = 0;

    // graph stores the number of edges connected to the node and the ids of the connected nodes
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::with_capacity(n as usize);
    for e in &edges {
        let (a,b) = (e[0], e[1]);
        graph.entry(a).and_modify(|v| {v.insert(b);} ).or_insert(HashSet::from([b]));
        graph.entry(b).and_modify(|v| {v.insert(a);} ).or_insert(HashSet::from([a]));
    }
    println!("initial graph: {graph:?}");

    // the nodes with only 1 edge are the leafs.
    // the leafs are either divisible by k (they are added as a component)
    // or they are not, in which case their value with move along its (only)
    // edge and be added to that nodes value.
    // once a node is 'consumed' remove the edge
    
    let mut queue: VecDeque<i32> = VecDeque::new();
    for (k,v) in graph.iter() {
        if v.len() == 1 { 
            queue.push_back(*k);
        }
    }
    println!("initial leaves: {queue:?}");

    while let Some(node) = queue.pop_front() {
        println!("looking at: {node}");
        if values[node as usize] % k == 0 {
            println!(" ~ ans + 1 ~ ");
            ans += 1;
        }
    
        // the last node will not have any neighbors
        if graph.get(&node).unwrap().len() == 0 { break };

        // get the nodes only neighbor (it must be a leaf) and do the updates
        let nei = *graph.get(&node).unwrap().iter().next().unwrap();
        values[nei as usize] = (values [nei as usize] + values[node as usize]).rem_euclid(k);
        graph.get_mut(&nei).unwrap().remove(&node);
        if graph.get(&nei).unwrap().len() == 1 {
            queue.push_back(nei);
        }
        println!(" -- updated graph: {graph:?}");
        println!(" -- updated queue: {queue:?}");
    }

    return ans;
}

#[test]
fn tests() {
    let n = 5;
    let edges = vec![vec![0,2],vec![1,2],vec![1,3],vec![2,4]];
    let values = vec![1,8,1,4,4];
    let k = 6;
    assert_eq!(2, max_k_divisible_components(n, edges, values, k));

    let n = 7;
    let edges = vec![vec![0,1],vec![0,2],vec![1,3],vec![1,4],vec![2,5],vec![2,6]];
    let values = vec![3,0,6,1,5,2,1];
    let k = 3;
    assert_eq!(3, max_k_divisible_components(n, edges, values, k));

    let n = 1;
    let edges = vec![];
    let values = vec![0];
    let k = 1;
    assert_eq!(1, max_k_divisible_components(n, edges, values, k));

    let n = 2;
    let edges = vec![vec![0,1]];
    let values = vec![4,3];
    let k = 7;
    assert_eq!(1, max_k_divisible_components(n, edges, values, k));
}
