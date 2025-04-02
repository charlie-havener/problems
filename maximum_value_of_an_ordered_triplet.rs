pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {

    let mut ans: i64 = 0;
    let mut store: Vec<i32> = vec![0; nums.len()];

    // first store the largest to the left
    store[0] = nums[0];
    for i in 1..nums.len() {
        store[i] = nums[i-1].max(store[i-1]);
    }

    // now replace with the largest to the right
    // calculate the 'score' before replacement
    store[nums.len() - 1] = nums[nums.len() - 1];
    for i in (0..nums.len() - 1).rev() {
        let l = nums[i+1].max(store[i+1]);
        ans = ans.max((store[i] as i64 - nums[i] as i64) * l as i64);
        store[i] = l;
    }

    return ans;
}


#[test]
fn tests() {
    let nums = vec![12,6,1,2,7];
    assert_eq!(77, maximum_triplet_value(nums));

    let nums = vec![1,10,3,4,19];
    assert_eq!(133, maximum_triplet_value(nums));

    let nums = vec![1,2,3];
    assert_eq!(0, maximum_triplet_value(nums));
}
