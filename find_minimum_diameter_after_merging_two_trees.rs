use std::collections::{HashMap, VecDeque};

pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
    let a = get_diameter(edges1);
    let b = get_diameter(edges2);
    return a.max(b).max((a+1)/2 + (b+1)/2 + 1);
}

fn get_diameter(edges: Vec<Vec<i32>>) -> i32 {

    if edges.len() == 0 { return 0 }
    if edges.len() == 1 { return 1 }

    let mut diameter = 0;
    let mut nodes = edges.len() + 1;
    let mut hm: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut in_degree: Vec<usize> = vec![0; nodes];

    for e in edges {
        let (a, b) = (e[0] as usize, e[1] as usize);
        hm.entry(a).and_modify(|v| v.push(b)).or_insert(vec![b]);
        hm.entry(b).and_modify(|v| v.push(a)).or_insert(vec![a]);
        in_degree[a as usize] += 1;
        in_degree[b as usize] += 1;
    }

    let mut queue = VecDeque::new();
    for (idx, v) in in_degree.iter().enumerate() {
        if *v == 1 {
            queue.push_back(idx);
        }
    }

    loop {
        let count = queue.len();
        for _ in 0..count {
            let t = queue.pop_front().unwrap();
            nodes -= 1;
            for nei in hm.get(&t).unwrap().iter() { 
                in_degree[*nei] -= 1;
                if in_degree[*nei] == 1 {
                    queue.push_back(*nei);
                }
            }
        }

        diameter += 1;

        if nodes - queue.len() == 0 {
            return if queue.len() == 1 { diameter * 2 } else { diameter * 2 + 1 }
        }

    }
}

#[test]
fn tests() {
    let edges1 = vec![vec![0,1],vec![0,2],vec![0,3]];
    let edges2 = vec![vec![0,1]];
    assert_eq!(3, minimum_diameter_after_merge(edges1, edges2));

    let edges1 = vec![vec![0,1],vec![0,2],vec![0,3],vec![2,4],vec![2,5],vec![3,6],vec![2,7]];
    let edges2 = vec![vec![0,1],vec![0,2],vec![0,3],vec![2,4],vec![2,5],vec![3,6],vec![2,7]];
    assert_eq!(5, minimum_diameter_after_merge(edges1, edges2));

    let edges1 = vec![];
    let edges2 = vec![];
    assert_eq!(1, minimum_diameter_after_merge(edges1, edges2));

    let edges1 = vec![vec![0,1]];
    let edges2 = vec![];
    assert_eq!(2, minimum_diameter_after_merge(edges1, edges2));

    let edges1 = vec![];
    let edges2 = vec![vec![0,1]];
    assert_eq!(2, minimum_diameter_after_merge(edges1, edges2));

    let edges1 = vec![vec![0,1]];
    let edges2 = vec![vec![0,1]];
    assert_eq!(3, minimum_diameter_after_merge(edges1, edges2));

    let edges1 = vec![vec![0,1],vec![2,0],vec![3,2],vec![3,6],vec![8,7],vec![4,8],vec![5,4],vec![3,5],vec![3,9]];
    let edges2 = vec![vec![0,1],vec![0,2],vec![0,3]];
    assert_eq!(7, minimum_diameter_after_merge(edges1, edges2));

}
