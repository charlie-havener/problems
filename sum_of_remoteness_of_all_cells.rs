fn dfs(row: usize, col: usize, grid: &mut Vec<Vec<i32>>, total: &mut i64) -> (i64, i64) {

    let nei = [(0,-1),(0,1),(1,0),(-1,0)];

    let mut size = 1;
    let mut total = grid[row][col] as i64;
    let mut queue = vec![(row, col)];
    grid[row][col] *= -1;

    while let Some((r,c)) = queue.pop() {
        
        for n in nei {
            let nr = r as i32 + n.0;
            let nc = c as i32 + n.1;
            if nr >= 0 && nr < grid.len() as i32 && nc >= 0 && nc < grid[0].len() as i32 && grid[nr as usize][nc as usize] > 0 {
                size += 1;
                total += grid[nr as usize][nc as usize] as i64;
                grid[nr as usize][nc as usize] *= -1;
                queue.push((nr as usize, nc as usize));
            }
        }
    }
    return (total, size);
}


pub fn sum_remoteness(mut grid: Vec<Vec<i32>>) -> i64 {
    
    let mut total: i64 = 0;
    let mut groups = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] < 0 { continue }
            let (t,s) = dfs(i, j, &mut grid, &mut total);
            total += t;
            groups.push((t,s));
        }
    }

    let mut ans = 0;
    for g in groups {
        ans += (total - g.0) * g.1;
    }

    return ans;
}

#[test]
fn tests() {
    let grid = vec![vec![-1,1,-1],vec![5,-1,4],vec![-1,3,-1]];
    assert_eq!(39, sum_remoteness(grid));

    let grid = vec![vec![-1,3,4],vec![-1,-1,-1],vec![3,-1,-1]];
    assert_eq!(13, sum_remoteness(grid));

    let grid = vec![vec![1]];
    assert_eq!(0, sum_remoteness(grid));
}
