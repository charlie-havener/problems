pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
    let mut ans = 1;
    let mut left = 0;
    let mut curr = nums[0];

    for i in 1..nums.len() {

        while left < i && curr & nums[i] != 0 {
            curr ^= nums[left];
            left += 1;
        }
        println!("{i}, {left}");
        ans = ans.max(i - left + 1);
        curr |= nums[i];

    }

    return ans as i32;
}
