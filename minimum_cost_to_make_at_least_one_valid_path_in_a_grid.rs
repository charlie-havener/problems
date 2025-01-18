use std::collections::VecDeque;
use std::cmp::Reverse;

pub fn min_cost(mut grid: Vec<Vec<i32>>) -> i32 {

    const NEI: [(i32,i32); 4] = [(0,1),(0,-1),(1,0),(-1,0)];

    let num_rows = grid.len() as i32;
    let num_cols = grid[0].len() as i32;

    let mut min_costs = vec![vec![i32::MAX; num_cols as usize]; num_rows as usize];
    min_costs[0][0] = 0;

    let mut dq = VecDeque::new();
    dq.push_front((0,0));

    while let Some((row, col)) = dq.pop_front() {

        if row == num_rows - 1 && col == num_cols - 1 { return min_costs[row as usize][col as usize] }

        for (idx, nei) in NEI.iter().enumerate() {
            let n_row = row + nei.0;
            let n_col = col + nei.1;
            let cost = if grid[row as usize][col as usize] == idx as i32 + 1 { 0 } else { 1 };

            if n_row >= 0 && n_row < num_rows && n_col >= 0 && n_col < num_cols
            && min_costs[n_row as usize][n_col as usize] > min_costs[row as usize][col as usize] + cost {
                min_costs[n_row as usize][n_col as usize] = min_costs[row as usize][col as usize] + cost;
                if cost == 1 {
                    dq.push_back((n_row, n_col));
                }
                else {
                    dq.push_front((n_row, n_col));
                }
            }
        }
    }
    unreachable!();
    //return min_costs[num_rows as usize - 1][num_cols as usize - 1];
}

#[test]
fn tests() {
    let grid = vec![vec![1,1,1,1],vec![2,2,2,2],vec![1,1,1,1],vec![2,2,2,2]];
    assert_eq!(3, min_cost(grid));

    let grid = vec![vec![1,1,3],vec![3,2,2],vec![1,1,4]];
    assert_eq!(0, min_cost(grid));

    let grid = vec![vec![1,2],vec![4,3]];
    assert_eq!(1, min_cost(grid));
}
