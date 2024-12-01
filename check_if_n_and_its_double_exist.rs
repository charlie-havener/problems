pub fn check_if_exists(arr: Vec<i32>) -> bool {
    
    let mut pos_pair = vec![false; 1001];
    let mut neg_pair = vec![false; 1001];
    for v in arr {
        if v < 0 {
            let t = v.abs() as usize;
            if neg_pair[t] { return true }
            if t % 2 == 0 { neg_pair[t/2] = true };
        }
        if v >= 0 {
            let t = v as usize;
            if pos_pair[t] { return true }
            if t % 2 == 0 { pos_pair[t/2] = true };
        }
    }
    return false;
}
