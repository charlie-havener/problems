pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {

    let mut max_sum = nums[0];
    let mut ans = max_sum;

    for i in 1..nums.len() {
        if nums[i-1] >= nums[i] {
            max_sum = nums[i];
        } else {
            max_sum += nums[i];
        }
        ans = ans.max(max_sum);
    }

    return ans;
}
