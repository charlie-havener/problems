pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {

    let maximum_val = 2_i32.pow(maximum_bit as u32) - 1;
    let mut ans = vec![maximum_val; nums.len()];
    let mut rolling_xor = 0;

    for idx in 0..nums.len() {
        rolling_xor ^= nums[idx];
        ans[nums.len() - 1 - idx] ^= rolling_xor;
    }

    return ans;
}

#[test]
fn tests() {
    let nums = vec![0,1,1,3];
    let max_bit = 2;
    assert_eq!(vec![0,3,2,3], get_maximum_xor(nums, max_bit));

    let nums = vec![2,3,4,7];
    let max_bit = 3;
    assert_eq!(vec![5,2,6,5], get_maximum_xor(nums, max_bit));

    let nums = vec![0,1,2,2,5,7];
    let max_bit = 3;
    assert_eq!(vec![4,3,6,4,6,7], get_maximum_xor(nums, max_bit));
}
