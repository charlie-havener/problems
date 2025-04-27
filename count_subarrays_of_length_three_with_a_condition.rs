pub fn count_subarrays(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    for r in 2..nums.len() {
        if (nums[r] + nums[r-2]) << 1 == nums[r-1] {
            ans += 1;
        }
    }
    return ans;
}
