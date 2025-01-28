pub fn find_max_fish(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;

    const NEI: [(i32,i32);4] = [(-1,0),(1,0),(0,-1),(0,1)];
    let row_count = grid.len();
    let col_count = grid[0].len();

    for row in 0..row_count {
        for col in 0..col_count {
            if grid[row][col] > 0 {
                ans = ans.max(dfs(row,col,&mut grid, &NEI));
            }
        }
    }


    return ans;
}

fn dfs(row: usize, col: usize, grid: &mut Vec<Vec<i32>>, dirs: &[(i32,i32); 4]) -> i32 {
    let mut count = 0;
    let mut queue: Vec<(usize, usize)> = Vec::new();
    queue.push((row, col));
    count += grid[row][col];
    grid[row][col] *= -1;

    while let Some((r,c)) = queue.pop() {
        for d in dirs {
            let nr = r as i32 + d.0;
            let nc = c as i32 + d.1;
            if nr >= 0 && nr < grid.len() as i32 && nc >= 0 && nc < grid[0].len() as i32 && grid[nr as usize][nc as usize] > 0 {
                count += grid[nr as usize][nc as usize];
                grid[nr as usize][nc as usize] *= -1;
                queue.push((nr as usize, nc as usize));
            }
        }
    }

    return count
}

#[test]
fn tests() {
    let grid = vec![vec![0,2,1,0],vec![4,0,0,3],vec![1,0,0,4],vec![0,3,2,0]];
    assert_eq!(7, find_max_fish(grid));

    let grid = vec![vec![1,0,0,0],vec![0,0,0,0],vec![0,0,0,0],vec![0,0,0,1]];
    assert_eq!(1, find_max_fish(grid));

    let grid = vec![vec![0,1,0],vec![1,1,1],vec![0,1,0]];
    assert_eq!(5, find_max_fish(grid));
}
