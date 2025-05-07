use std::collections::BinaryHeap;
use std::cmp::Reverse;


pub fn min_time_to_reach(mut move_time: Vec<Vec<i32>>) -> i32 {

    const NEI: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // the bottom right corner of the grid
    let target = (move_time.len() - 1, move_time[0].len() - 1);

    let mut bh: BinaryHeap<(Reverse<i32>, i32, i32)>  = BinaryHeap::new();
    bh.push((Reverse(0), 0, 0));

    while let Some((Reverse(time), row, col)) = bh.pop() {
        for (row_offset, col_offset) in NEI {
            let next_row = row + row_offset;
            let next_col = col + col_offset;

            if is_valid(next_row, next_col, &move_time) {
                let next_row = next_row as usize;
                let next_col = next_col as usize;

                // '+1' since it takes one second to travel
                let next_time = time.max(move_time[next_row][next_col]) + 1;

                if next_row == target.0 && next_col == target.1 { return next_time }

                bh.push((Reverse(next_time), next_row as i32, next_col as i32));
                move_time[next_row][next_col] = move_time[next_row][next_col] * -1 - 1;

            }
        }
    }

    return -1;
}


// helper function for determing if a row is valid.
// valid meaning in bounds and not already visited.
fn is_valid(row: i32, col: i32, move_time: &Vec<Vec<i32>>) -> bool {
    if row < 0 { return false }
    if row >= move_time.len() as i32 { return false }
    if col < 0 { return false }
    if col >= move_time[0].len() as i32 { return false }
    if move_time[row as usize][col as usize] < 0 { return false }
    return true;
}
