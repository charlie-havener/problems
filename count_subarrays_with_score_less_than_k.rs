pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {

    let mut left = 0;
    let mut right = 0;
    let mut sum: i64 = 0;
    let mut ans: i64 = 0;

    while right < nums.len() {
        
        sum += nums[right] as i64;
        while (right as i64 - left as i64 + 1) * sum >= k{
            ans += (right - left) as i64;
            sum -= nums[left] as i64;
            left += 1;
        }
        right += 1; 
    }
    ans += ((right - left)*(right - left + 1) / 2) as i64; 

    return ans;
}

#[test]
fn tests() {
    let nums = vec![2,1,4,3,5];
    let k = 10;
    assert_eq!(6, count_subarrays(nums, k));

    let nums = vec![1,1,1];
    let k = 5;
    assert_eq!(5, count_subarrays(nums, k));

    let nums = vec![2,8,8,8,8];
    let k = 6;
    assert_eq!(1, count_subarrays(nums, k));

    let nums = vec![2,2,2,2,2,2,2];
    let k = 999_999_999_999_999;
    assert_eq!(28, count_subarrays(nums, k));
}
