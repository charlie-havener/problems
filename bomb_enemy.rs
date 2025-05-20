pub fn max_killed_enemies(grid: Vec<Vec<char>>) -> i32 {

    let mut store = vec![vec![0; grid[0].len()]; grid.len()];
    let mut ans = 0;

    for row in 0..grid.len() {

        let mut total = 0;
        for col in 0..grid[0].len() {
            match grid[row][col] {
                'W' => total = 0,
                'E' => total += 1,
                '0' => store[row][col] += total,
                _ => panic!("??"),
            }
        }

        total = 0;
        for col in (0..grid[0].len()).rev() {
            match grid[row][col] {
                'W' => total = 0,
                'E' => total += 1,
                '0' => store[row][col] += total,
                _ => panic!("??"),
            }
        }

    }

    for col in 0..grid[0].len() {

        let mut total = 0;
        for row in 0..grid.len() {
            match grid[row][col] {
                'W' => total = 0,
                'E' => total += 1,
                '0' => store[row][col] += total,
                _ => panic!("??"),
            }
        }

        total = 0;
        for row in (0..grid.len()).rev() {
            match grid[row][col] {
                'W' => total = 0,
                'E' => total += 1,
                '0' => store[row][col] += total,
                _ => panic!("??"),
            }
        }

    }

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            ans = ans.max(store[row][col]);
        }
    }

    return ans;
}
