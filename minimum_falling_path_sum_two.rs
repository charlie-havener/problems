pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    if grid.len() == 1 { return grid[0][0] };

    let mut ms = [0,0];
    let mut prev = vec![0; grid.len()];
    let mut curr = grid[0].clone();

    for r in 0..grid.len() {
        let mut ms_new = [i32::MAX, i32::MAX];
        for c in 0..grid.len() {
            curr[c] = grid[r][c];
            if prev[c] == ms[0] {
                curr[c] += ms[1];
            } else {
                curr[c] += ms[0];
            }
            
            if curr[c] <= ms_new[0] {
                ms_new[1] = ms_new[0];
                ms_new[0] = curr[c];
            } else if curr[c] < ms_new[1] {
                ms_new[1] = curr[c];
            }
        }

        ms = ms_new;
        std::mem::swap(&mut curr, &mut prev);
    }
    return ms[0];
}

#[test]
fn tests() {
    let grid = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
    assert_eq!(13, min_falling_path_sum(grid));

    let grid = vec![vec![7]];
    assert_eq!(7, min_falling_path_sum(grid));
}

