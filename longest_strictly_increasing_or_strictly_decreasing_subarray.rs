pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {

    let mut up_len = 1;
    let mut down_len = 1;
    let mut ans = 1;

    for p in nums.windows(2) {
        let (a,b) = (p[0],p[1]);
        if a < b {
            up_len += 1;
            down_len = 1;
        }
        if a > b {
            down_len += 1;
            up_len = 1;
        }
        if a == b {
            up_len = 1;
            down_len = 1;
        }
        ans = ans.max(up_len).max(down_len);
    }
    return ans;
}
