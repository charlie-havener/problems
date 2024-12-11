pub fn maximum_beauty(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    let mut left = 0;
    let mut right = 0;
    let mut ans = 0;

    while right < nums.len() {
        while nums[left] + 2 * k < nums[right] {
            left += 1;
        }
        ans = ans.max(right - left + 1);
        right += 1;
    }

    return ans as i32;
}

#[test]
fn tests() {
    let nums = vec![4,6,1,2];
    let k = 2;
    assert_eq!(3, maximum_beauty(nums, k));

    let nums = vec![1,1,1,1];
    let k = 10;
    assert_eq!(4, maximum_beauty(nums, k));
}
