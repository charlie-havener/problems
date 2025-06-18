pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {

    nums.sort_unstable();
    let mut ans: Vec<Vec<i32>> = Vec::with_capacity(nums.len()/3 + 1);

    for c in nums.chunks_exact(3) {
        if c[2] - c[0] > k { return vec![] }
        ans.push(c.to_vec());
    }

    return ans;
}
