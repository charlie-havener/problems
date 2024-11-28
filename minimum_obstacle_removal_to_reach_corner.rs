use std::collections::BinaryHeap;
use std::cmp::Reverse;

// pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
// 
//     const NEI: [(i32,i32); 4] = [(-1,0), (1,0), (0,-1), (0,1)];
// 
//     let (m,n) = (grid.len(), grid[0].len());
// 
//     let mut visited = vec![vec![i32::MAX; n]; m];
//     visited[0][0] = grid[0][0];
// 
//     let mut pq: BinaryHeap<(Reverse<i32>, i32, i32)> = BinaryHeap::new();
//     pq.push((Reverse(visited[0][0]), 0, 0));
// 
//     while let Some((Reverse(removals), row, col)) = pq.pop() {
//         
//         // the first time we get to the end is with the lowest # of removals
//         if row as usize == m-1 && col as usize == n-1 { return removals }
// 
//         for nxt in NEI {
//             let (nr, nc) = (row + nxt.0, col + nxt.1);
//             
//             // check that the neighbor is not out of bounds
//             if !(nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32) { continue }
// 
//             // only add to queue if it will be a new min for that position
//             let needed_removals = removals + grid[nr as usize][nc as usize];
//             if needed_removals < visited[nr as usize][nc as usize] {
//                 visited[nr as usize][nc as usize] = needed_removals;
//                 pq.push((Reverse(needed_removals), nr, nc));
//             }
//         }
//     }
//     unreachable!();
// }


// 0-1 BFS
use std::collections::VecDeque;
pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {

    const NEI: [(i32,i32); 4] = [(-1,0), (1,0), (0,-1), (0,1)];
    
    let (m,n) = (grid.len(), grid[0].len());
    let mut visited = vec![vec![false; n]; m];
    visited[0][0] = true;

    let mut pq: VecDeque<(i32,i32,i32)> = VecDeque::new();
    pq.push_front((0,0,0));

    while let Some((removals, row, col)) = pq.pop_front() {
        
        for nxt in NEI {
            let (nr, nc) = (row + nxt.0, col + nxt.1);

            // make sure the next cell is valid
            if !(nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 && !visited[nr as usize][nc as usize]) { continue }

            if nr as usize == m-1 && nc as usize == n-1 {
                return removals;
            }

            visited[nr as usize][nc as usize] = true;
            if grid[nr as usize][nc as usize] == 1 {
                pq.push_back((removals+1,nr,nc));
            } else {
                pq.push_front((removals,nr,nc));
            }
        }

        println!("{pq:?}");
    }
    unreachable!();
}

#[test]
fn tests() {
    let grid = vec![vec![0,1,1],vec![1,1,0],vec![1,1,0]];
    assert_eq!(2, minimum_obstacles(grid));

    let grid = vec![vec![0,1,0,0,0],vec![0,1,0,1,0],vec![0,0,0,1,0]];
    assert_eq!(0, minimum_obstacles(grid));

    let grid = vec![vec![0,1,1],vec![1,1,1],vec![1,1,0]];
    assert_eq!(3, minimum_obstacles(grid));
}
