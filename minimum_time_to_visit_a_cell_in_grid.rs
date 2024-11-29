use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {

    if grid[0][1] > 1 && grid[1][0] > 1 { return -1 }

    let (rows, cols) = (grid.len(), grid[0].len());
    const NEI: [(i32,i32);4] = [(-1,0),(1,0),(0,-1),(0,1)];
    
    let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];
    visited[0][0] = true;
    let mut pq: BinaryHeap<(Reverse<i32>, i32, i32)> = BinaryHeap::new();
    pq.push((Reverse(0), 0, 0));

    while let Some((Reverse(time), row, col)) = pq.pop() {
        for nxt in NEI {
            let (nr, nc) = (row + nxt.0, col + nxt.1);
            
            // check that the neighbor is valid
            // => in bounds and unvisited
            if nr < 0 || nr as usize >= rows ||
               nc < 0 || nc as usize >= cols ||
               visited[nr as usize][nc as usize]
            {
                continue
            }
            
            let wait = match (grid[nr as usize][nc as usize] - time) % 2 == 0 {
                true => 1,
                false => 0,
            };
            let nt = (time+1).max(grid[nr as usize][nc as usize] + wait);
            if nr as usize == rows - 1 && nc as usize == cols - 1 { return nt }
            visited[nr as usize][nc as usize] = true;
            pq.push((Reverse(nt), nr, nc))
        }
    }
    unreachable!();
}

#[test]
fn tests() {
    let grid = vec![vec![0,1,3,2],vec![5,1,2,5],vec![4,3,8,6]];
    assert_eq!(7, minimum_time(grid));

    let grid = vec![vec![0,2,4],vec![3,2,1],vec![1,0,4]];
    assert_eq!(-1, minimum_time(grid));

    let grid = vec![vec![0,0],vec![12,12]];
    assert_eq!(12, minimum_time(grid));
}
