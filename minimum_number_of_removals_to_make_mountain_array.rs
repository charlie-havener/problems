pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        
    let mut counts: Vec<i32> = vec![1; nums.len()];

    for i in 0..nums.len() {
        for j in 0..i {
            if nums[j] < nums[i] {
                counts[i] = counts[i].max(counts[j] + 1);
            }
        }
    }

    let mut ans = i32::MAX;
    for i in (0..nums.len()).rev() {
        let tmp = counts[i];
        counts[i] = 1;
        for j in (i+1)..nums.len() {
            if nums[j] < nums[i] {
                counts[i] = counts[i].max(counts[j] + 1);
            }
        }

        if counts[i] >= 2 && tmp >= 2 {
            ans = ans.min(nums.len() as i32 - (counts[i] + tmp - 1));
        }

    }

    return ans;
}


#[test]
fn tests() {
    let nums = vec![1,3,1];
    assert_eq!(0, minimum_mountain_removals(nums));

    let nums = vec![2,1,1,5,6,2,3,1];
    assert_eq!(3, minimum_mountain_removals(nums));
}
