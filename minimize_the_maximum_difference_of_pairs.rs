pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {

    nums.sort_unstable();

    let mut left = 0;
    let mut right = *nums.last().unwrap() - nums[0];

    for idx in 0..nums.len() - 1 {
        nums[idx] = nums[idx+1] - nums[idx];
    }
    nums.pop();

    while left <= right {
        let mid = (right - left)/2 + left;
        if possible_check(mid, &nums, p) {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    return left;
}

fn possible_check(v: i32, nums: &Vec<i32>, target: i32) -> bool {
    let mut count = 0;
    let mut i = 0;
    while i < nums.len() {
        if nums[i] <= v {
            count += 1;
            i += 1;
        }
        if count >= target {
            return true;
        }
        i += 1;
    }
    return false;
}

