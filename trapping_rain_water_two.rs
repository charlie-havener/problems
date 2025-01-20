use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn trap_rain_water(mut height_map: Vec<Vec<i32>>) -> i32 {
    let num_rows = height_map.len();
    let num_cols = height_map[0].len();

    if num_rows <= 2 || num_cols <= 2 { return 0 }

    const NEI: [(i32,i32); 4] = [(-1,0),(1,0),(0,-1),(0,1)];
    let mut ans = 0;

    let mut pq: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();

    // add boundaries to the pq and mark them as visited
    for row in 1..(num_rows-1) {
        pq.push((Reverse(height_map[row][0]), row, 0));
        pq.push((Reverse(height_map[row][num_cols-1]), row, num_cols - 1));
        height_map[row][0] = (height_map[row][0] * -1) - 1;
        height_map[row][num_cols-1] = (height_map[row][num_cols -1] * -1) - 1;
    }
    for col in 1..(num_cols-1) {
        pq.push((Reverse(height_map[0][col]), 0, col));
        pq.push((Reverse(height_map[num_rows-1][col]), num_rows-1, col));
        height_map[0][col] = (height_map[0][col] * -1) - 1;
        height_map[num_rows-1][col] = (height_map[num_rows-1][col] * -1) - 1;
    }

    // mark cornes as visited
    height_map[0][0] = (height_map[0][0] * -1) - 1;
    height_map[0][num_cols-1] = (height_map[0][num_cols-1] * -1) - 1;
    height_map[num_rows-1][0] = (height_map[num_rows-1][0] * -1) - 1;
    height_map[num_rows-1][num_cols-1] = (height_map[num_rows-1][num_cols-1] * -1) - 1;

    while let Some((Reverse(height), row, col)) = pq.pop() {
        for nei in NEI {
            let n_row = row as i32 + nei.0;
            let n_col = col as i32 + nei.1;

            // make sure the neighbor is in bounds and not visited
            if n_row >= 0 && n_row < num_rows as i32 && n_col >= 0 && n_col < num_cols as i32 && height_map[n_row as usize][n_col as usize] >= 0 {
                ans += 0.max(height - height_map[n_row as usize][n_col as usize]);
                pq.push((Reverse(height.max(height_map[n_row as usize][n_col as usize])), n_row as usize, n_col as usize));
                height_map[n_row as usize][n_col as usize] = (height_map[n_row as usize][n_col as usize] * -1) - 1;
            }
        }
    }

    return ans;
}

#[test]
fn tests() {
    let height_map = vec![vec![1,4,3,1,3,2],vec![3,2,1,3,2,4],vec![2,3,3,2,3,1]];
    assert_eq!(4, trap_rain_water(height_map));

    let height_map = vec![vec![3,3,3,3,3],vec![3,2,2,2,3],vec![3,2,1,2,3],vec![3,2,2,2,3],vec![3,3,3,3,3]];
    assert_eq!(10, trap_rain_water(height_map));

    let height_map = vec![vec![12,13,1,12],vec![13,4,13,12],vec![13,8,10,12],vec![12,13,12,12],vec![13,13,13,13]];
    assert_eq!(14, trap_rain_water(height_map));
}
