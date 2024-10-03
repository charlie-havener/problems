use std::collections::HashMap;

pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
    let mut total = 0;
    for v in &nums {
        total += *v as i64;
    }
    let remainder = total.rem_euclid(p as i64) as i32;
    if remainder == 0 { return 0 }

    let mut last_seen = HashMap::new();
    last_seen.insert(0, -1);
    let mut ans = nums.len() as i32;

    let mut prefix = 0;
    for i in 0..nums.len() {
        prefix = (prefix + nums[i]).rem_euclid(p);
        let comp = (prefix - remainder).rem_euclid(p);
        if let Some(loc) = last_seen.get(&comp) {
            ans = ans.min(i as i32 - loc);
        }
        last_seen.insert(prefix, i as i32);
    }
    return if ans == nums.len() as i32 { -1 } else { ans };
}

#[test]
fn tests() {
    
    let nums = vec![3,1,4,2];
    let p = 6;
    assert_eq!(1, min_subarray(nums, p));

    let nums = vec![6,3,5,2];
    let p = 9;
    assert_eq!(2, min_subarray(nums, p));

    let nums = vec![1,2,3];
    let p = 3;
    assert_eq!(0, min_subarray(nums, p));

    let nums = vec![3,3,4,3,4,3];
    let p = 3;
    assert_eq!(3, min_subarray(nums, p));

}
