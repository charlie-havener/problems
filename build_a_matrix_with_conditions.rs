use std::collections::VecDeque;
use std::collections::HashMap;

struct TopoSort {
    graph: HashMap<i32, Vec<i32>>,
    degrees: Vec<i32>,
}

impl TopoSort {
    fn new(k: i32) -> Self {
        Self {
            graph: HashMap::new(),
            degrees: vec![0; k as usize + 1],
        }
    }
}

pub fn build_matrix(k: i32, row_conditions: Vec<Vec<i32>>, col_conditions: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    
    // transform the conditions into a directed graph
    let to_graph = |conditions: &Vec<Vec<i32>>| -> TopoSort {
        conditions.iter().fold(TopoSort::new(k), |mut acc, c| {
            acc.graph.entry(c[0]).and_modify(|v| v.push(c[1])).or_insert(vec![c[1]]);
            acc.degrees[c[1] as usize] += 1;
            acc
        })
        
    };
    let TopoSort{graph: row_graph, degrees: mut row_degrees} = to_graph(&row_conditions);
    let TopoSort{graph: col_graph, degrees: mut col_degrees} = to_graph(&col_conditions);

    
    // topological sort on rows to get some valid row #s for each [1,k] (if they exist)
    let mut row_nums = vec![0; k as usize + 1];
    let mut pq = row_degrees[1..].iter().enumerate().fold(VecDeque::new(), |mut acc, (idx, v)| {
        if *v == 0 { acc.push_back(idx as i32 + 1) }
        acc
    });
    if pq.len() == 0 { return vec![] } // whole graph is cyclic
    let mut count = 0;
    while let Some(v) = pq.pop_front() {
        row_nums[v as usize] = count;
        if let Some(dest) = row_graph.get(&v) {
            for d in dest {
                row_degrees[*d as usize] -= 1;
                if row_degrees[*d as usize] == 0 {
                    pq.push_back(*d)
                }
            }
        }
        count += 1;
    }
    if count != k { return vec![] } // if not everything was added (there was a cycle)

    // topologically sort on cols to get some valid col #s for each [1,k] (if they exist)
    let mut col_nums = vec![0; k as usize + 1];
    let mut pq = col_degrees[1..].iter().enumerate().fold(VecDeque::new(), |mut acc, (idx, v)|{
        if *v == 0 { acc.push_back(idx as i32 + 1) }
        acc
    });
    if pq.len() == 0 { return vec![] } // whole graph is cyclic
    let mut count = 0;
    while let Some(v) = pq.pop_front() {
        col_nums[v as usize] = count;
        if let Some(dest) = col_graph.get(&v) {
            for d in dest {
                col_degrees[*d as usize] -= 1;
                if col_degrees[*d as usize] == 0 {
                    pq.push_back(*d)
                }
            }
        }
        count += 1;
    }
    if count != k { return vec![] } // if not everything was added (there was a cycle)

    // consruct the answer
    let mut ans: Vec<Vec<i32>> = vec![vec![0; k as usize]; k as usize];
    for i in 1..=k {
        let r = row_nums[i as usize];
        let c = col_nums[i as usize];
        ans[r as usize][c as usize] = i;
    }

    return ans;
}

#[test]
fn tests() {

    let k = 3;
    let rs = vec![vec![1,2],vec![3,2]];
    let cs = vec![vec![2,1],vec![3,2]];
    println!("{:?}", build_matrix(k,rs,cs));

    let k = 3;
    let rs = vec![vec![1,2],vec![2,3],vec![3,1],vec![2,3]];
    let cs = vec![vec![2,1]];
    assert_eq!(Vec::<Vec<i32>>::new(), build_matrix(k,rs,cs));

}
