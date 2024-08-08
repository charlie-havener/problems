pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {

    let mut ans: Vec<Vec<i32>> = Vec::with_capacity((rows*cols) as usize);

    let mut right = c_start + 1;
    let mut left = c_start - 1;
    let mut bot = r_start + 1;
    let mut top = r_start - 1;
    let mut init = (r_start, c_start + 1);

    ans.push(vec![r_start, c_start]);
    while ans.len() < (rows * cols) as usize {

        if right < cols {
            for r in (init.0).max(0)..(bot+1).min(rows) {
                ans.push(vec![r, right]);
            }
        }
        init.0 = bot;

        if bot < rows {
            for c in (left.max(0)..(init.1).min(cols)).rev() {
                ans.push(vec![bot, c]);
            }
        }
        init.1 = left;

        if left >= 0 {
            for r in (top.max(0)..(init.0).min(rows)).rev() {
                ans.push(vec![r, left]);
            }
        }
        init.0 = top;

        if top >= 0 {
            for c in (init.1 + 1).max(0)..=right.min(cols-1) {
                ans.push(vec![top, c]);
            }
        }
        init.1 = right;

        init.1 += 1;
        right += 1;
        left -= 1;
        bot += 1;
        top -= 1;
    }
    return ans;
}

#[test]
fn tests() {
    assert_eq!(vec![vec![0,0],vec![0,1],vec![0,2],vec![0,3]], spiral_matrix_iii(1,4,0,0));

    assert_eq!(vec![vec![1,4],vec![1,5],vec![2,5],vec![2,4],vec![2,3],vec![1,3],vec![0,3],vec![0,4],vec![0,5],vec![3,5],vec![3,4],vec![3,3],vec![3,2],vec![2,2],vec![1,2],vec![0,2],vec![4,5],vec![4,4],vec![4,3],vec![4,2],vec![4,1],vec![3,1],vec![2,1],vec![1,1],vec![0,1],vec![4,0],vec![3,0],vec![2,0],vec![1,0],vec![0,0]], spiral_matrix_iii(5,6,1,4));

    assert_eq!(vec![vec![3,4],vec![3,5],vec![4,5],vec![4,4],vec![4,3],vec![3,3],vec![2,3],vec![2,4],vec![2,5],vec![4,2],vec![3,2],vec![2,2],vec![1,2],vec![1,3],vec![1,4],vec![1,5],vec![4,1],vec![3,1],vec![2,1],vec![1,1],vec![0,1],vec![0,2],vec![0,3],vec![0,4],vec![0,5],vec![4,0],vec![3,0],vec![2,0],vec![1,0],vec![0,0]], spiral_matrix_iii(5,6,3,4));
}
