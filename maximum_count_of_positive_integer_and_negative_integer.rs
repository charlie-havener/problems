pub fn maximum_count(nums: Vec<i32>) -> i32 {

    let mut left = -1;
    let mut right = nums.len() as i32;
    while right - left > 1 {
        let m = (right - left) / 2 + left;
        if nums[m as usize] > 0 {
            right = m;
        } else {
            left = m;
        }
    }
    // left is the index of the last number less than or equal to 0
    // or -1 if all numbers are greater than 0
    let p_count = nums.len() as i32 - left - 1;

    left = -1;
    right = nums.len() as i32;
    while right - left > 1 {
        let m = (right - left) / 2 + left;
        if nums[m as usize] >= 0 {
            right = m;
        } else {
            left = m;
        }
    }
    // left is index of the last number less than 0
    // or -1 if all numbers are greater than or equal to 0
    let n_count = left + 1;
    
    return n_count.max(p_count);
}

#[test]
fn tests() {
    let nums = vec![-2,-1,-1,1,2,3];
    println!("{}", maximum_count(nums));

    let nums = vec![-2,-1,-1,0,0,1,2,3];
    println!("{}", maximum_count(nums));

    let nums = vec![-3,-2,-1,-1];
    println!("{}", maximum_count(nums));

    let nums = vec![-3,-2,-1,-1,0,0];
    println!("{}", maximum_count(nums));

    let nums = vec![1,2,3,4];
    println!("{}", maximum_count(nums));
    
    let nums = vec![0,0,1,2,3,4];
    println!("{}", maximum_count(nums));

    let nums = vec![-2,-1,-1,0,0,1,1,1,2,3];
    println!("{}", maximum_count(nums));
}
