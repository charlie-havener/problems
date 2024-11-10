pub fn mimimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    
    let (mut left, mut right) = (0,0);
    let mut or_value = 0;
    let mut ans = usize::MAX;

    while right < nums.len() {
        //
        // exit early in min possible answer is found
        if ans == 1 { return 1 }

        or_value |= nums[right];
        while or_value >= k {
            ans = ans.min(right - left + 1);
            if left == right { break }
            or_value ^= nums[left];
            left += 1;
        }

        right += 1;
    }

    if ans == usize::MAX { return -1 }
    return ans as i32;
}

#[test]
fn tests() {
    let nums = vec![1,2,3];
    let k = 2;
    assert_eq!(1, mimimum_subarray_length(nums, k));

    let nums = vec![2,1,8];
    let k = 10;
    assert_eq!(3, mimimum_subarray_length(nums, k));

    let nums = vec![1,2];
    let k = 0;
    assert_eq!(1, mimimum_subarray_length(nums, k));

    let nums = vec![2,1,8,10];
    let k = 10;
    assert_eq!(1, mimimum_subarray_length(nums, k));
}
