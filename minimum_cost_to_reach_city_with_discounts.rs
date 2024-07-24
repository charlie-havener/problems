use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn minimum_cost(n: i32, highways: Vec<Vec<i32>>, discounts: i32) -> i32 {
    
    let discounts = discounts as usize;

    let edges = highways.iter().fold(vec![vec![] ; n as usize], |mut acc, h| {
        let city1 = h[0] as usize;
        let city2 = h[1] as usize;
        let toll = h[2];
        
        acc[city1].push((city2, toll));
        acc[city2].push((city1, toll));

        acc
    });
    
    let mut visited = vec![ vec![i32::MAX; n as usize] ; discounts + 1 as usize];
    visited[discounts][0] = 0;

    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0),discounts,0)); // cost, discounts, city

    while !pq.is_empty() {

        let (cost, discounts, city) = pq.pop().unwrap();

        if city == n as usize - 1 { return cost.0 }

        if cost.0 > visited[discounts][city] { continue }

        for (nei, toll) in edges[city].iter() {
            // with no discount
            let c = cost.0 + toll;
            if c < visited[discounts][*nei] {
                pq.push((Reverse(c), discounts, *nei));
                visited[discounts][city] = c;
            }
            
            // with discount applied if able
            if discounts > 0 {
                let c = cost.0 + toll/2;
                if c < visited[discounts-1][*nei] {
                    pq.push((Reverse(c), discounts-1, *nei));
                    visited[discounts - 1][city] = c;
                }
            }
        }
    }
    
    return -1;
}

#[test]
fn tests() {
    let n = 5;
    let h = vec![vec![0,1,4],vec![2,1,3],vec![1,4,11],vec![3,2,3],vec![3,4,2]];
    let d = 1;
    assert_eq!(9, minimum_cost(n,h,d));
    
    let n = 4;
    let h = vec![vec![1,3,17],vec![1,2,7],vec![3,2,5],vec![0,1,6],vec![3,0,20]];
    let d = 20;
    assert_eq!(8, minimum_cost(n,h,d));

    let n = 4;
    let h = vec![vec![0,1,3],vec![2,3,2]];
    let d = 0;
    assert_eq!(-1, minimum_cost(n,h,d));
}
