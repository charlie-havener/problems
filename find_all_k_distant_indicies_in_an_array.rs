pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
    let mut res = Vec::new();
    let mut r = 0;
    for j in 0..nums.len() {
        if nums[j] == key {
            let l = r.max(j as i32 - k);
            r = (nums.len() as i32 - 1).min(j as i32 + k) + 1;
            for i in l..r {
                res.push(i);
            }
        }
    }
    res
}
