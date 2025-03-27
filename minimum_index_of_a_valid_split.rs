pub fn minimum_index(nums: Vec<i32>) -> i32 {

    // find the dominant element
    let mut dom = 0;
    let mut count = 1;
    for idx in 1..nums.len() {
        if nums[idx] == nums[dom] {
            count += 1;
        }
        if count * 2 <= idx - dom + 1 {
            dom = idx;
            count = 1;
        }
    }

    // get the number of occurances of the dominant element
    let dom = nums[dom];
    let dom_count = nums.iter().filter(|e| **e == dom).count();

    // check for ans
    count = 0;
    for (idx, val) in nums[..nums.len()-1].iter().enumerate() {
        if *val != dom { continue }
        count += 1;
        let left = idx + 1;
        let right = nums.len() - left;
        if count * 2 > left && (dom_count - count) * 2 > right {
            return idx as i32;
        }
    }
    
    return -1;
}

#[test]
fn tests() {
    let nums = vec![1,2,2,2];
    assert_eq!(2, minimum_index(nums));

    let nums = vec![2,1,3,1,1,1,7,1,2,1];
    assert_eq!(4, minimum_index(nums));

    let nums = vec![3,3,3,3,7,2,2];
    assert_eq!(-1, minimum_index(nums));
}
