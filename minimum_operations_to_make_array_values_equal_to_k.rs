pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut min_set = i32::MAX;
    let mut set_count = 0;
    let mut set = [false; 101];
    for n in nums {
        if set[n as usize] == false { set_count += 1 }
        set[n as usize] = true;
        if n < min_set { min_set = n }
    }

    if k > min_set { return - 1 }
    if k == min_set { return set_count - 1 }
    return set_count;
}
