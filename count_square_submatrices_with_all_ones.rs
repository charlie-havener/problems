pub fn count_squares(mut matrix: Vec<Vec<i32>>) -> i32 {

    let mut ans = 0;
    let (rows, cols) = (matrix.len(), matrix[0].len());
    for r in 0..rows {
        for c in 0..cols {
            if r == 0 || c == 0 {
                ans += matrix[r][c];
            }
            else if matrix[r][c] == 1 {
                let x = matrix[r-1][c-1];
                let y = matrix[r-1][c];
                let z = matrix[r][c-1];
                let val = x.min(y.min(z)) + 1;
                ans += val;
                matrix[r][c] = val;
            }
        }
    }
    return ans;
}

#[test]
fn tests() {
    let m = vec![vec![0,1,1,1],vec![1,1,1,1],vec![0,1,1,1]];
    assert_eq!(15, count_squares(m));

    let m = vec![vec![1,0,1],vec![1,1,0],vec![1,1,0]];
    assert_eq!(7, count_squares(m));

    let m = vec![vec![1]];
    assert_eq!(1, count_squares(m));

    let m = vec![vec![0]];
    assert_eq!(0, count_squares(m));

    let m = vec![vec![0,1,0,1,1]];
    assert_eq!(3, count_squares(m));
    let m = vec![vec![1],vec![1],vec![0],vec![1],vec![0]];
    assert_eq!(3, count_squares(m));

}
