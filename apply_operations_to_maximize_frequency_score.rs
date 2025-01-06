pub fn max_frequency_score(mut nums: Vec<i32>, k: i64) -> i32 {

    nums.sort_unstable();
    let mut ps: Vec<i64> = vec![0; nums.len() + 1];
    let mut l = 0;
    let mut ans = 1;

    for r in 0..nums.len() {
        ps[r+1] = ps[r] + nums[r] as i64;
        loop {
            let mid = (l + r) / 2;
            let l_diff = nums[mid] as i64 * (mid - l) as i64 - (ps[mid] - ps[l]) as i64;
            let r_diff = (ps[r+1] - ps[mid+1]) as i64 - nums[mid] as i64 * (r - mid) as i64;
            if l_diff + r_diff <= k { break }
            l += 1;
        }
        ans = ans.max(r - l + 1)
    }

    return ans as i32;
}

#[test]
fn tests() {
    let nums = vec![1,2,6,4];
    let k = 3;
    assert_eq!(3, max_frequency_score(nums, k));

    let nums = vec![1,4,4,2,4];
    let k = 0;
    assert_eq!(3, max_frequency_score(nums, k));

    let nums = vec![3,20,13,2,3,15,24,19,8,13,19,20,21];
    let k = 45;
    assert_eq!(10, max_frequency_score(nums, k));
}
