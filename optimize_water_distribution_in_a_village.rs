use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn min_cost_to_supply_water(n: i32, wells: Vec<i32>, pipes: Vec<Vec<i32>>) -> i32 {
    // Each tuple is (cost, from, to)
    let mut pq: BinaryHeap<(Reverse<i32>, i32, i32)> = BinaryHeap::new();
    for idx in 1..=n {
        pq.push((Reverse(wells[idx as usize - 1]), idx, idx));
    }

    let pipes = pipes.iter().fold(HashMap::new(), |mut acc: HashMap<i32, Vec<(i32, i32)>>, p| {
        acc.entry(p[0]).and_modify(|v| v.push((p[1],p[2]))).or_insert(vec![(p[1],p[2])]);
        acc.entry(p[1]).and_modify(|v| v.push((p[0],p[2]))).or_insert(vec![(p[0],p[2])]);
        acc
    });
    
    let mut has_water = vec![false; n as usize];
    let mut count = n; // for early return
    let mut sum = 0;


    // only items in the pq are ones that would lead to a water connection.
    while !pq.is_empty() {
        let (Reverse(cost), from, to) = pq.pop().unwrap(); // pq not empty, so unwrap is safe
        
        match (has_water[from as usize - 1], has_water[to as usize - 1]) {
            (true, true) => (), // both are already connected, edges were already added;
            _ => {
                sum += cost;
                for home in [from, to] {
                    if !has_water[home as usize - 1] { 
                        has_water[home as usize - 1] = true;
                        count -= 1;
                        
                        // add the new edges to the pq
                        if let Some(ps) = pipes.get(&home) {
                            for p in ps {
                                pq.push((Reverse(p.1), home, p.0));
                            }
                        }
                        


                    }
                }
            }
        }
        if count == 0 { return sum }
    }
    return sum; // should be unreachable since we can at a minimum build a well at each house.
}


pub fn min_cost_to_supply_water2(n: i32, wells: Vec<i32>, pipes: Vec<Vec<i32>>) -> i32 {
    let mut edges = pipes;
    for idx in 1..=n {
        edges.push(vec![-1*idx, idx, wells[idx as usize-1]]);
    }
    edges.sort_unstable_by(|a,b| a[2].cmp(&b[2]));
    println!("{:?}", edges);

    let mut visited = vec![false; n as usize];
    let mut count = n; // for early exit
    let mut sum = 0;

    for e in edges {
        let (from, to, cost) = (e[0], e[1], e[2]);
        // dummy nodes have a value equal to -1 * [n] => less than 0
        if from < 0 {
            if !visited[to as usize - 1] {
                sum += cost;
                visited[to as usize - 1] = true;
                count += 1;
            }
        } else {
            match (!visited[to as usize - 1], !visited[from as usize - 1]) {
                (false, false) => (), // both have already been visited
                (true, false) | (false, true) => {
                    sum += cost;
                    visited[to as usize - 1] = true;
                    visited[from as usize - 1] = true;
                    count += 1;
                },
                (true, true) => {
                    sum += cost;
                    visited[to as usize - 1] = true;
                    visited[from as usize - 1] = true;
                    count += 2;
                }
            }
        }

        if count == n { return sum }
    }

    return sum; // should be unreachable since we can, at a minimum, build a well at each house.
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let n = 3;
        let wells = vec![1,2,2];
        let pipes = vec![vec![1,2,1],vec![2,3,1]];
        assert_eq!(3, min_cost_to_supply_water(n, wells, pipes));
    }
    #[test]
    fn test2() {
        let n = 2;
        let wells = vec![1,1];
        let pipes = vec![vec![1,2,1],vec![1,2,2]];
        assert_eq!(2, min_cost_to_supply_water(n, wells, pipes));
    }
}
