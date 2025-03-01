pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {

    for i in 0..(nums.len() - 1) {
        if nums[i] == nums[i + 1] {
            nums[i] *= 2;
            nums[i + 1] = 0;
        }
    }
    
    let mut zptr = 0;
    for i in 0..nums.len() {
        if nums[zptr] != 0 {
            zptr += 1;
            continue;
        }
        
        if nums[i] == 0 {
            continue;
        }

        nums.swap(i, zptr);
        zptr += 1;

    }

    return nums;
}


#[test]
fn tests() {
    let mut nums = vec![1,2,2,1,1,0];
    assert_eq!(vec![1,4,2,0,0,0], apply_operations(nums));

    let nums = vec![0,1];
    assert_eq!(vec![1,0], apply_operations(nums));

    let nums = vec![1,2,3,4,0,0,0];
    assert_eq!(vec![1,2,3,4,0,0,0], apply_operations(nums));
    
    let nums = vec![0,0,0,1,2,3,4];
    assert_eq!(vec![1,2,3,4,0,0,0], apply_operations(nums));

    let nums = vec![0,1,0,0,2,3,0];
    assert_eq!(vec![1,2,3,0,0,0,0], apply_operations(nums));

    let nums = vec![1,0,2,3,0,0,4];
    assert_eq!(vec![1,2,3,4,0,0,0], apply_operations(nums));
}
