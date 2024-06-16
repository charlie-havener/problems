pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
    let mut ans = 0;
    let mut i = 0;
    let mut miss: i64 = 1;
    while miss as i32 <= n {
        if i < nums.len() && (nums[i] as i64) < miss {
            miss += nums[i] as i64;
            i += 1;
        }
        else {
            miss += miss;
            ans += 1;
        }
    }
    return ans;
}
