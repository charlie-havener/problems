pub fn find_lucky(arr: Vec<i32>) -> i32 {
    let mut counts = [0; 501];
    counts[0] = -1; // nums in [1,500] so it'd be 0,0 and return 0 (not what we want)
    for a in arr {
        counts[a as usize] += 1;
    }
    for (idx, &cnt) in counts.iter().enumerate().rev() {
        if cnt as usize == idx { return cnt }
    }
    return -1
}
