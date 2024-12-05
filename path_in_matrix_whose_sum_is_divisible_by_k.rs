pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {

    const MOD: i32 = 1_000_000_007;

    let mut curr: Vec<Vec<i32>> = vec![vec![0;k as usize]; grid[0].len() + 1];
    let mut prev = curr.clone();
    prev[1][0] = 1;

    for row in grid {
        for (idx,val) in row.into_iter().enumerate() {

            for i in 0..k as usize {
                let cidx = (i as i32 + val).rem_euclid(k) as usize;
                curr[idx + 1][cidx] = (curr[idx][i] + prev[idx + 1][i]).rem_euclid(MOD);
            }
        }
        std::mem::swap(&mut curr, &mut prev);
    }
    return prev.last().unwrap()[0];
}

#[test]
fn tests() {
    let grid = vec![vec![5,2,4],vec![3,0,5],vec![0,7,2]];
    let k = 3;
    assert_eq!(2, number_of_paths(grid, k));

    let grid = vec![vec![0,0]];
    let k = 5;
    assert_eq!(1, number_of_paths(grid, k));

    let grid = vec![vec![7,3,4,9],vec![2,3,6,2],vec![2,3,7,0]];
    let k = 1;
    assert_eq!(10, number_of_paths(grid, k));
}
