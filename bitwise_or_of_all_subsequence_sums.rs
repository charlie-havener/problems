pub fn subsequence_sum_or(nums: Vec<i32>) -> i64 {
    nums.into_iter().fold((0_i64,0_i64), |(mut a, mut b), n| {
        b+=n as i64;
        a |= b | n as i64;
        (a,b)
    }).0
}
