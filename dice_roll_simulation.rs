pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {

    const MOD: i32 = 1_000_000_007;
    let n = n as usize;
    let m: usize = *roll_max.iter().max().unwrap() as usize + 1;

    // store the row sum, followed by the counts of each die roll at their respective idx
    let mut dp: [[i32;7];16] = [[1;7]; 16];
    dp[0][0] = 6;

    for i in 1..n {
        let row = i % m;
        
        for die in 1..=6 {
            let prev_row = ((row as i32 - 1).rem_euclid(m as i32)) as usize;
            let prev_sum = dp[prev_row][0];
            let offset = roll_max[die - 1] as usize;
            if offset > i {
                dp[row][die] = prev_sum;
            }
            else if offset == i {
                dp[row][die] = prev_sum - 1;
            } else {
                let offrow = (i - offset - 1) % m;
                dp[row][die] = (prev_sum - (dp[offrow][0] - dp[offrow][die])).rem_euclid(MOD);
            }
        }

        dp[row][0] = 0;
        for die in 1..=6 {
            dp[row][0] = (dp[row][0] + dp[row][die]).rem_euclid(MOD);
        }

    }
    return dp[((n-1) % m) as usize][0];
}

#[test]
fn tests() {
    let n = 2;
    let roll_max = vec![1,1,2,2,2,3];
    assert_eq!(34, die_simulator(n, roll_max));

    let n = 2;
    let roll_max = vec![1,1,1,1,1,1];
    assert_eq!(30, die_simulator(n, roll_max));

    let n = 3;
    let roll_max = vec![1,1,1,2,2,3];
    assert_eq!(181, die_simulator(n, roll_max));

    let n = 8;
    let roll_max = vec![1,1,1,2,2,3];
    assert_eq!(891800, die_simulator(n, roll_max));

    let n = 2634;
    let roll_max = vec![13,11,1,6,9,4];
    assert_eq!(233266655, die_simulator(n, roll_max));

    let n = 100;
    let roll_max = vec![7,5,15,5,1,7];
    assert_eq!(797209093, die_simulator(n, roll_max));
}
