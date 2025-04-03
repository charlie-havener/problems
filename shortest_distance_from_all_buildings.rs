use std::collections::VecDeque;

pub fn shortest_distance(mut grid: Vec<Vec<i32>>) -> i32 {

    let mut tracker = 0;
    let mut scores = vec![vec![0; grid[0].len()]; grid.len()];
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            // do a search from each of the houses to all the empties
            if grid[row][col] == 1 {
                bfs(&mut grid, row, col, tracker, &mut scores);
                tracker -= 1;
            }
        }
    }

    let mut ans = None;
    for row in 0..scores.len() {
        for col in 0..scores[0].len() {
            if grid[row][col] == tracker {
                ans = match ans {
                    None => Some(scores[row][col]),
                    Some(v) => Some(v.min(scores[row][col])),
                }
            }
        }
    }

    return ans.unwrap_or(-1);
}


fn bfs(grid: &mut Vec<Vec<i32>>, row: usize, col: usize, tracker: i32, scores: &mut Vec<Vec<i32>>) {

    const NEI: [(i32, i32); 4] = [(-1,0), (1,0), (0,-1), (0,1)];

    let mut queue = VecDeque::new();
    queue.push_back((row, col, 0));

    while let Some((r, c, dist)) = queue.pop_front() {
        scores[r][c] += dist;
        for (rc, cc) in NEI {
            let (nr, nc) = (r as i32 + rc, c as i32 + cc);
            if nr >= 0 && nr < grid.len() as i32 &&
            nc >= 0 && nc < grid[0].len() as i32 &&
            grid[nr as usize][nc as usize] == tracker {

                grid[nr as usize][nc as usize] -= 1;
                queue.push_back((nr as usize, nc as usize, dist + 1));
            }
        }
    }
}


#[test]
fn tests() {
    let grid = vec![vec![1,0,2,0,1],vec![0,0,0,0,0],vec![0,0,1,0,0]];
    assert_eq!(7, shortest_distance(grid));

    let grid = vec![vec![1,0]];
    assert_eq!(1, shortest_distance(grid));

    let grid = vec![vec![1]];
    assert_eq!(-1, shortest_distance(grid));
}
