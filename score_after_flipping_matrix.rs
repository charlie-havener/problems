pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
    fn flip_row(row: usize, g: &mut Vec<Vec<i32>>) {
        for v in &mut g[row] {
            *v ^= 1;
        }
    }
    
    fn flip_col(col: usize, g: &mut Vec<Vec<i32>>) {
        for row in 0..g.len() {
            g[row][col] ^= 1;
        }
    }

    for idx in 0..grid.len() {
        if grid[idx][0] == 0 {
            flip_row(idx, &mut grid);
        }
    }

    for col in 1..grid[0].len() {
        let mut count = 0;
        for row in 0..grid.len() {
            count += grid[row][col];
        }
        if count <= (grid.len() / 2) as i32 {
            flip_col(col, &mut grid);
        }
    }

    let mut ans = 0;
    for row in grid {
        ans += row.iter().fold(0, |acc, digit| (acc << 1) + digit);
    }

    return ans;
}

#[test]
fn tests() {

    let g = vec![vec![0,0,1,1],vec![1,0,1,0],vec![1,1,0,0]];
    assert_eq!(39, matrix_score(g));

    let g = vec![vec![0]];
    assert_eq!(1, matrix_score(g));
}
