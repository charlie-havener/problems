pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {

    let s = grid[0].iter().map(|v| *v as i64).sum::<i64>();

    let mut up = s - grid[0][0] as i64;
    let mut down = 0;
    let mut ans = down.max(up);

    for idx in 1..grid[0].len() {
        up -= grid[0][idx] as i64;
        down += grid[1][idx - 1] as i64;
        ans = ans.min(down.max(up));
    }

    return ans;
}
