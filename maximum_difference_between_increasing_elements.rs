pub fn maximum_difference(nums: Vec<i32>) -> i32 {

    let mut min = nums[0];
    let mut ans = -1;
    for n in &nums[1..] {
        if *n <= min {
            min = *n;
        }
        else {
            ans = ans.max(*n - min);
        }
    }
    return ans;
}
