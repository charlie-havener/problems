use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;


pub fn min_cost(n: i32, roads: Vec<Vec<i32>>, apple_cost: Vec<i32>, k: i32) -> Vec<i64> {
    let mut ans: Vec<i64> = apple_cost.iter().map(|v| *v as i64).collect::<Vec<_>>();
    let edges: HashMap<i32, Vec<(usize, i32)>> = roads.into_iter().fold(HashMap::new(), |mut acc, r| {
        let w = r[2] * (k + 1);
        acc.entry(r[0]).and_modify(|v| v.push((r[1] as usize, w))).or_insert(vec![(r[1] as usize, w)]);
        acc.entry(r[1]).and_modify(|v| v.push((r[0] as usize, w))).or_insert(vec![(r[0] as usize, w)]);
        acc
    });
    println!("edges: {:?}", edges);

    let mut pq = BinaryHeap::new();
    for idx in 0..apple_cost.len() {
        pq.push((Reverse(ans[idx]), idx + 1));
    }
    println!("stating pq: {:?}", pq);


    while let Some((_, u)) = pq.pop() {
        if edges.get(&(u as i32)).is_none() { continue }
        for (v, w) in edges.get(&(u as i32)).unwrap() {
            let w = *w as i64;
            if ans[*v - 1] > ans[u - 1] + w {
                ans[*v - 1] = ans[u - 1] + w;
                pq.push((Reverse(ans[*v - 1]), *v));
            }
        }
    }




    return ans;
}

#[test]
fn test() {
    let n = 4;
    let roads = vec![vec![1,2,4],vec![2,3,2],vec![2,4,5],vec![3,4,1],vec![1,3,4]];
    let apple_cost = vec![56,42,102,301];
    let k = 2;
    assert_eq!(vec![54,42,48,51], min_cost(n,roads,apple_cost,k));
}

