use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {

    const MOD: i32 = 1_000_000_007;

    let graph: Vec<Vec<(usize,i64)>> = roads
        .iter()
        .fold(
            vec![vec![]; n as usize],
            |mut acc, r| {
                let (source, dest, weight) = (r[0] as usize, r[1] as usize, r[2] as i64);
                acc[source].push((dest, weight));
                acc[dest].push((source, weight));
                acc
            }
        );

    let mut shorts: Vec<i64> = vec![i64::MAX; n as usize];
    let mut counts: Vec<i32> = vec![0; n as usize];
    shorts[0] = 0;
    counts[0] = 1;

    let mut pq: BinaryHeap<(Reverse<i64>, usize)> = BinaryHeap::from([(Reverse(0), 0)]);
    while let Some((Reverse(time), node)) = pq.pop() {
        if shorts[node] < time { continue }

        for (nei, tme) in &graph[node] {
            if shorts[*nei] == time + *tme {
                counts[*nei] = (counts[*nei] + counts[node]).rem_euclid(MOD);
            }
            else if shorts[*nei] > time + *tme {
                shorts[*nei] = time + *tme;
                counts[*nei] = counts[node];
                pq.push((Reverse(shorts[*nei]), *nei));
            }
        }
    }

    return *counts.last().unwrap();
}
