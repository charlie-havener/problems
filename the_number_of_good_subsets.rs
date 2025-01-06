pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
    
    const MOD: i64 = 1_000_000_007;
    let primes = [2,3,5,7,11,13,17,19,23,29];
    let good_numbers = [2,3,5,6,7,10,11,13,14,15,17,19,21,22,23,26,29,30];
    let counts = nums.iter().fold([0;31], |mut acc, n| { acc[*n as usize] += 1; acc });

    let mut dp: [i64; 1025] = [0; 1025];
    dp[0] = 1;

    for n in good_numbers {
        let p_mask: usize = primes.iter().enumerate().filter(|(_,p)| n % **p == 0).map(|(idx, _)| 1 << idx).sum();
        for idx in 0..(1<<10) {
            if idx & p_mask != 0 { continue }
            let mask = idx | p_mask as usize;
            dp[mask] = (dp[mask] + counts[n as usize] * dp[idx]).rem_euclid(MOD);
        }
    }

    let mut ans = dp.iter().skip(1).fold(0, |acc, v| { (acc + v).rem_euclid(MOD) });
    for _ in 1..counts[1] {
        ans = (ans << 1).rem_euclid(MOD);
    }
    return ans as i32;
}

#[test]
fn tests() {
    let nums =  vec![1,2,3,4];
    assert_eq!(6, number_of_good_subsets(nums));

    let nums =  vec![4,2,3,15];
    assert_eq!(5, number_of_good_subsets(nums));

    let nums =  vec![1,1,1,2];
    assert_eq!(8, number_of_good_subsets(nums));
}
