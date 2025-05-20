pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
    let mut store = vec![0; nums.len() + 1];
    let mut t = 0;
    for q in queries {
        store[q[0] as usize] += 1;
        store[q[1] as usize + 1] -= 1;
    }
    for i in 0..nums.len() {
        t += store[i];
        if t < nums[i] { return false }
    }
    return true;
}
