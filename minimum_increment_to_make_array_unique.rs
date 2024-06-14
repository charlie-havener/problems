pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();

    let mut ans = 0;
    for i in 1..nums.len() {
        if nums[i] <= nums[i-1] {
            ans += nums[i-1] + 1 - nums[i];
            nums[i] = nums[i-1] + 1;
        }
    }

    return ans;
}

#[test]
fn tests() {
    let v = vec![1,2,2];
    assert_eq!(1, min_increment_for_unique(v));
    
    let v = vec![3,2,1,2,1,7];
    assert_eq!(6, min_increment_for_unique(v));

    let v = vec![0,0,0,0];
    assert_eq!(6, min_increment_for_unique(v));

    let v = vec![2,2,2,2];
    assert_eq!(6, min_increment_for_unique(v));

    let v = vec![2,2,3,4,5,6,7,8];
    assert_eq!(7, min_increment_for_unique(v));

    let v = vec![2,2,2,2,3,4,6,6];
    assert_eq!(17, min_increment_for_unique(v));
}
