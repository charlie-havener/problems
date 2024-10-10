pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
    
    let mut nums: Vec<(i32, usize)> = nums.into_iter().enumerate().fold(Vec::new(), |mut acc, (idx, v)| {
        acc.push((v, idx));
        acc
    });
    nums.sort_unstable();
    
    let mut ans = 0;
    let mut min_idx = nums[0].1;
    for (_, idx) in nums {
        if idx > min_idx {
            ans = ans.max(idx - min_idx);
        } 
        else {
            min_idx = idx;
        }
    }
    return ans as i32;
}

#[test]
fn tests() {
    let v = vec![6,0,8,2,1,5];
    assert_eq!(4, max_width_ramp(v));

    let v = vec![9,8,1,0,1,9,4,0,4,1];
    assert_eq!(7, max_width_ramp(v));
}
