pub fn check(nums: Vec<i32>) -> bool {

    if nums.len() == 1 { return true }

    //make sure it wraps
    let mut down_jump = if nums.last().unwrap() <= &nums[0] { false } else { true };

    for pair in nums.windows(2) {
        let a = pair[0];
        let b = pair[1];

        if a <= b { continue }
        else {
            if !down_jump { down_jump = true }
            else { return false; }
        }
    }
    return true;

}
