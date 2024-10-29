pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {

    let mut prev: Vec<bool> = vec![true; grid.len()];
    let mut curr: Vec<bool> = vec![false; grid.len()];

    let mut col = 1;
    while col < grid[0].len() {
        //println!("{col}:\n {prev:?}");
        let mut go_next = false;
        
        for r in 0..grid.len() {
            let value = grid[r][col];
            curr[r] = false;

            // upper left
            if r != 0 {
                curr[r] |= grid[r-1][col-1] < value && prev[r-1];
            }

            // left
            curr[r] |= grid[r][col-1] < value && prev[r];

            // lower left
            if r + 1 < grid.len() {
                curr[r] |= grid[r+1][col-1] < value && prev[r+1];
            }

            // there would be a valid cell in the column
            if curr[r] {
                go_next = true;
            }
        }

        if !go_next { break }

        std::mem::swap(&mut curr, &mut prev);
        col += 1;
    }


    //println!("{col}:\n {prev:?}");
    return col as i32 - 1;
}

#[test]
fn tests() {
    let grid = vec![vec![2,4,3,5],vec![5,4,9,3],vec![3,4,2,11],vec![10,9,13,15]];
    assert_eq!(3, max_moves(grid));

    let grid = vec![vec![3,2,4],vec![2,1,9],vec![1,1,7]];
    assert_eq!(0, max_moves(grid));
}
