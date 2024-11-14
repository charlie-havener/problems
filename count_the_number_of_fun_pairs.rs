pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
    
    nums.sort_unstable();
    let mut ans: i64 = 0;

    for i in 0..nums.len() - 1 {
        
        let (s1, s2);

        // bsearch for the low point
        let seek = lower - nums[i];
        let mut left = i;
        let mut right = nums.len();
        while right - left > 1 {
            let mid = left + (right - left) / 2;
            if seek <= nums[mid] { right = mid }
            else { left = mid }
        }
        s1 = left;

        // bsearch for the high point
        let seek = upper - nums[i];
        let mut left = i;
        let mut right = nums.len();
        while right - left > 1 {
            let mid = left + (right - left) / 2;
            if seek < nums[mid] { right = mid }
            else { left = mid }
        }
        s2 = left;
        
        ans += (s2 - s1) as i64;
    }

    return ans;
}


#[test]
fn tests() {
    let nums = vec![0,1,7,4,4,5];
    let lower = 3;
    let upper = 6;
    assert_eq!(6, count_fair_pairs(nums, lower, upper));

    let nums = vec![1,7,9,2,5];
    let lower = 11;
    let upper = 11;
    assert_eq!(1, count_fair_pairs(nums, lower, upper));

    let nums = vec![1,1,1,1,1,1,1];
    let lower = 0;
    let upper = 20;
    assert_eq!(21, count_fair_pairs(nums, lower, upper));
}
