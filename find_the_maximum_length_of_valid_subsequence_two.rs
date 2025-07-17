pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {

    let mut dp = [[0; 1_000]; 1_000];
    let mut ans = 0;

    for n in nums {
        let n = (n % k) as usize;
        for i in 0..(k as usize) {
            dp[i][n] = dp[n][i] + 1;
            ans = ans.max(dp[i][n]);
        }
    }

    return ans;
}
