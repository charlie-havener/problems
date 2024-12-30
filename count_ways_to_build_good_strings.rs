pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {

    const MOD: i32 = 1_000_000_007;
    let mut ans: i32 = 0;

    let mut dp = vec![0; high as usize + 1];
    dp[0] = 1;

    for idx in 0..=high as usize {
        if idx >= low as usize {
            ans = (ans + dp[idx]).rem_euclid(MOD);
        }

        // add 'zero' zeros
        let nxt = idx + zero as usize;
        if nxt <= high as usize {
            dp[nxt] = (dp[nxt] + dp[idx]).rem_euclid(MOD);
        }

        // add 'one' ones
        let nxt = idx + one as usize;
        if nxt <= high as usize {
            dp[nxt] = (dp[nxt] + dp[idx]).rem_euclid(MOD);
        }
    }

    return ans;
}

#[test]
fn tests() {
    let low = 3;
    let high = 3;
    let zero = 1;
    let one = 1;
    assert_eq!(8, count_good_strings(low, high, zero, one));

    let low = 2;
    let high = 3;
    let zero = 1;
    let one = 2;
    assert_eq!(5, count_good_strings(low, high, zero, one));

    let low = 2;
    let high = 100000;
    let zero = 1;
    let one = 1;
    assert_eq!(215447029, count_good_strings(low, high, zero, one));

    let low = 2;
    let high = 100000;
    let zero = 1;
    let one = 2;
    assert_eq!(846671949, count_good_strings(low, high, zero, one));

}
