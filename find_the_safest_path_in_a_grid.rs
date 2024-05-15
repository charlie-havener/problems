pub fn maximum_safeness_factor(mut grid: Vec<Vec<i32>>) -> i32 {
    // set distances/visited and queue up all the robber locations
    let mut queue = Vec::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 1 {
                queue.push((r,c));
            }
            grid[r][c] -= 1; // 1 -> 0, 0 -> -1
        }
    }

    // BFS to add min distance to robbers in each cell
    // keep track of max distance value
    let mut largest_distance = 0;
    loop {
        let mut n_queue = Vec::new();
        while let Some((r,c)) = queue.pop() {
            largest_distance = largest_distance.max(grid[r][c]);
            if r > 0 && grid[r-1][c] == -1 {
                grid[r-1][c] = grid[r][c] + 1;
                n_queue.push((r-1,c));
            }
            if c > 0 && grid[r][c-1] == -1 {
                grid[r][c-1] = grid[r][c] + 1;
                n_queue.push((r,c-1));
            }
            if r + 1 < grid.len() && grid[r+1][c] == -1 {
                grid[r+1][c] = grid[r][c] + 1;
                n_queue.push((r+1,c));
            }
            if c + 1 < grid[0].len() && grid[r][c+1] == -1 {
                grid[r][c+1] = grid[r][c] + 1;
                n_queue.push((r,c+1));
            }
        }

        if n_queue.is_empty() { break; }
        std::mem::swap(&mut queue, &mut n_queue);
    }

    // binary search from 0 to largest_distance for the max safety score
    let mut left = 0;
    let mut right = largest_distance;
    let mut ans = 0;
    while left <= right {
        let mid = (right - left) / 2 + left;

        let mut visited = vec![false; grid.len() * grid[0].len()];
        let mut queue = vec![(0,0)];
        let mut made_to_end = false;

        loop {
            
            if grid[0][0] < mid || grid[grid.len()-1][grid[0].len()-1] < mid {
                right = (mid - 1).max(0);
                break;
            }

            let mut n_queue = Vec::new();
            while let Some((r,c)) = queue.pop() { 
                if visited[r * grid[0].len() + c] { continue; }
                visited[r * grid[0].len() + c] = true;

                // got to the end => it is possible with this mid value
                if r == grid.len() - 1 && c == grid[0].len() - 1 {
                    made_to_end = true;
                    break;
                }
                if r > 0 && grid[r-1][c] >= mid {
                    n_queue.push((r-1,c));
                }
                if c > 0 && grid[r][c-1] >= mid {
                    n_queue.push((r,c-1));
                }
                if r + 1 < grid.len() && grid[r+1][c] >= mid {
                    n_queue.push((r+1,c));
                }
                if c + 1 < grid[0].len() && grid[r][c+1] >= mid {
                    n_queue.push((r,c+1));
                }
            }

            if made_to_end {
                ans = mid;
                left = mid + 1;
                break;
            }
            if n_queue.is_empty() {
                right = (mid - 1).max(0);
                break;
            }
            std::mem::swap(&mut queue, &mut n_queue);
        }
    }
    return ans;
}

#[test]
fn tests() {
    let grid = vec![vec![1,0,0],vec![0,0,0],vec![0,0,1]];
    assert_eq!(0, maximum_safeness_factor(grid));

    let grid = vec![vec![0,0,1],vec![0,0,0],vec![0,0,0]];
    assert_eq!(2, maximum_safeness_factor(grid));

    let grid = vec![vec![0,0,0,1],vec![0,0,0,0],vec![0,0,0,0],vec![1,0,0,0]];
    assert_eq!(2, maximum_safeness_factor(grid));

    let grid = vec![vec![1,1,1],vec![0,1,1],vec![0,0,0]];
    assert_eq!(0, maximum_safeness_factor(grid));

    let grid = vec![vec![0,1,1],vec![0,0,1],vec![1,0,0]];
    assert_eq!(1, maximum_safeness_factor(grid));
}


