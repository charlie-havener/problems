use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn get_food(mut grid: Vec<Vec<char>>) -> i32 {
    
    const NEI: [(i32,i32);4] = [(-1,0),(1,0),(0,-1),(0,1)];

    let num_rows = grid.len() as i32;
    let num_cols = grid[0].len() as i32;

    let mut start: (usize, usize) = (0,0);
    for r in 0..num_rows {
        for c in 0..num_cols {
            if grid[r as usize][c as usize] == '*' {
                start = (r as usize, c as usize);
            }
        }
    }

    let mut pq: BinaryHeap<(Reverse<i32>, i32, i32)> = BinaryHeap::new();
    pq.push((Reverse(0), start.0 as i32, start.1 as i32));
    grid[start.0][start.1] = 'v';

    // mark visited cells by changing grid cell value to 'v'
    while let Some((Reverse(dist), row, col)) = pq.pop() {

        for nei in NEI {
            let n_row = row + nei.0;
            let n_col = col + nei.1;
            //println!(" check {n_row},{n_col}");
            //println!(" -{},{},{},{}", n_row >= 0, n_row < num_rows, n_col >= 0, n_col < num_cols);

            if n_row >= 0 && n_row < num_rows && n_col >= 0 && n_col < num_cols &&
            (grid[n_row as usize][n_col as usize] == 'O' || grid[n_row as usize][n_col as usize] == '#') {

                if grid[n_row as usize][n_col as usize] == '#' { return dist + 1 }

                grid[n_row as usize][n_col as usize] = 'v';
                pq.push((Reverse(dist + 1), n_row, n_col));
            }
        }
    }

    return -1;
}


#[test]
fn tests() {
    let grid = vec![vec!['X','X','X','X','X','X'],vec!['X','*','O','O','O','X'],vec!['X','O','O','#','O','X'],vec!['X','X','X','X','X','X']];
    assert_eq!(3, get_food(grid));

    let grid = vec![vec!['X','X','X','X','X'],vec!['X','*','X','O','X'],vec!['X','O','X','#','X'],vec!['X','X','X','X','X']];
    assert_eq!(-1, get_food(grid));

    let grid = vec![vec!['X','X','X','X','X','X','X','X'],vec!['X','*','O','X','O','#','O','X'],vec!['X','O','O','X','O','O','X','X'],vec!['X','O','O','O','O','#','O','X'],vec!['X','X','X','X','X','X','X','X']];
    assert_eq!(6, get_food(grid));
}
