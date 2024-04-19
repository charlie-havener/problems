pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    let visited: char = '2';
    let mut ans = 0;
        
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            
            if grid[row][col] == '1' {
                ans += 1;
                let mut stack = vec![(row, col)];

                while let Some((row,col)) = stack.pop() {
                    if grid[row][col] == visited {
                        continue;
                    }
                    grid[row][col] = visited;

                    // up
                    if row > 0 && grid[row-1][col] == '1' {
                        stack.push((row-1, col));
                    }

                    // down
                    if row < grid.len() - 1 && grid[row+1][col] == '1' {
                        stack.push((row+1, col));
                    }

                    // left
                    if col > 0 && grid[row][col-1] == '1' {
                        stack.push((row, col-1));
                    }

                    // right
                    if col < grid[0].len() - 1 && grid[row][col+1] == '1' {
                        stack.push((row, col+1));
                    }
                }
            }
            grid[row][col] = visited;
        }
    }
    return ans;
}

#[test]
fn tests() {
    let grid = vec![vec!['1','1','1','1','0'],vec!['1','1','0','1','0'],vec!['1','1','0','0','0'],vec!['0','0','0','0','0']];
    assert_eq!(1, num_islands(grid));

    let grid = vec![vec!['1','1','0','0','0'],vec!['1','1','0','0','0'],vec!['0','0','1','0','0'],vec!['0','0','0','1','1']];
    assert_eq!(3, num_islands(grid));

    let grid = vec![vec!['0']];
    assert_eq!(0, num_islands(grid));

    let grid = vec![vec!['1']];
    assert_eq!(1, num_islands(grid));



}

