pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let mut ans: i64 = 0;
    let mut left = 0;
    let mut right = 0;
    let mut count = 0;
    let largest = nums.iter().max().unwrap().clone();


    if nums[0] == largest { count += 1 }

    while right < nums.len() { // maybe can do something like len() - k ??
        if count == k {
            ans += (nums.len() - right) as i64;
            if nums[left] == largest { count -= 1 }
            left += 1;
        } else {
            right += 1;
            if right < nums.len() && nums[right] == largest {
                count += 1;
            }
        }
    }
    return ans;
}

#[test]
fn test() {
    let v = vec![1,3,2,3,3];
    let k = 2;
    assert_eq!(6, count_subarrays(v,k));

    let v = vec![1,4,2,1];
    let k = 3;
    assert_eq!(0, count_subarrays(v,k));
    
    let v = vec![1,3,2,3,0];
    let k = 2;
    assert_eq!(4, count_subarrays(v,k));
}
