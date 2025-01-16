use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn swim_in_water(mut grid: Vec<Vec<i32>>) -> i32 {

    const NEI: [(i32, i32); 4] = [(0,1), (0,-1), (1,0), (-1,0)];
    let n = grid.len() as i32;

    let mut ans = 0;
    let mut pq: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
    pq.push((Reverse(grid[0][0]), 0, 0));

    while let Some((Reverse(v), r, c)) = pq.pop() {

        ans = ans.max(v);
        for nei in NEI {
            let (nr, nc) = (r as i32 + nei.0, c as i32 + nei.1);
            if nr >= 0 && nr < n && nc >= 0 && nc < n && grid[nr as usize][nc as usize] >= 0 {
                if nr == (n-1) && nc == (n-1) {
                    return ans.max(grid[nr as usize][nc as usize]);
                }
                pq.push((Reverse(grid[nr as usize][nc as usize]), nr as usize, nc as usize));
                grid[nr as usize][nc as usize] *= -1;
            }
        }
    }

    unreachable!();
}

#[test]
fn tests() {
    let grid = vec![vec![0,2],vec![1,3]];
    assert_eq!(3, swim_in_water(grid));

    let grid = vec![vec![0,1,2,3,4],vec![24,23,22,21,5],vec![12,13,14,15,16],vec![11,17,18,19,20],vec![10,9,8,7,6]];
    assert_eq!(16, swim_in_water(grid));
}
