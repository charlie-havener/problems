pub fn min_moves(mut rooks: Vec<Vec<i32>>) -> i32 {

    let mut rows = vec![0; rooks.len()];
    let mut cols = vec![0; rooks.len()];

    for r in &rooks {
        rows[r[0] as usize] += 1;
        cols[r[1] as usize] += 1;
    }

    let mut ans = 0;
    let mut r: i32 = 0;
    let mut c: i32 = 0;

    for i in 0..rooks.len() {
        r += rows[i] - 1;
        c += cols[i] - 1;
        ans += r.abs() + c.abs()
    }

    return ans;
}

#[test]
fn tests() {
    let r = vec![vec![0,0],vec![1,0],vec![1,1]];
    assert_eq!(3, min_moves(r));

    let r = vec![vec![0,0],vec![0,1],vec![0,2],vec![0,3]];
    assert_eq!(6, min_moves(r));
}
