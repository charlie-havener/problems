pub fn divide_array(nums: Vec<i32>) -> bool {

    let mut counts: Vec<bool> = vec![true; 501];
    
    for n in nums {
        counts[n as usize] ^= true;
    }
    
    for c in counts {
        if c == false { return false }
    }
    return true;
}
