pub fn find_different_binary_string(nums: Vec<String>) -> String {
    
    let mut ans = String::with_capacity(nums[0].len());
    for (idx,n) in nums.into_iter().enumerate() {
        if &n[idx..=idx] == "0" {
            ans.push('1');
        } else {
            ans.push('0');
        }
    }
    return ans;
}
