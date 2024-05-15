pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let mut visited = vec![false; grid.len() * grid[0].len()];

    fn recurse(row: usize, col: usize, visited: &mut Vec<bool>, grid: &Vec<Vec<i32>>) -> i32 {
        if grid[row][col] == 0 { return 0 }
        if visited[row * grid[0].len() + col] { return 0 }
        visited[row * grid[0].len() + col] = true;
        let mut res = 0;
        if row > 0 {
            res = res.max(recurse(row - 1, col, visited, grid));
        }
        if col > 0 {
            res = res.max(recurse(row, col - 1, visited, grid));
        }
        if row < grid.len() - 1 {
            res = res.max(recurse(row + 1, col, visited, grid));
        }
        if col < grid[0].len() - 1 {
            res = res.max(recurse(row, col + 1, visited, grid));
        }
        visited[row * grid[0].len() + col] = false;
        return res + grid[row][col];

    }

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            ans = ans.max(
                recurse(r, c, &mut visited, &grid)
            );
        }
    }

    return ans;
}

#[test]
fn tests() {

    let grid = vec![vec![0,6,0],vec![5,8,7],vec![0,9,0]];
    assert_eq!(24, get_maximum_gold(grid));

    let grid = vec![vec![1,0,7],vec![2,0,6],vec![3,4,5],vec![0,3,0],vec![9,0,20]];
    assert_eq!(28, get_maximum_gold(grid));

}
