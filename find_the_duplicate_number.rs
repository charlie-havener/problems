pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
    let mut idx = 0;
    loop {
        if nums[idx] == idx as i32 + 1 {
            idx += 1;
        } else {
            let n = nums[idx] as usize - 1;
            let t = nums[n];
    
            if t == nums[idx] {
                return nums[idx];
            }

            nums[n] = nums[idx];
            nums[idx] = t;
        }
    }
}

#[test]
fn tests() {
    let v = vec![1,3,4,2,2];
    assert_eq!(2, find_duplicate(v));
    let v = vec![3,1,3,4,2];
    assert_eq!(3, find_duplicate(v));
    let v = vec![3,3,3,3,3];
    assert_eq!(3, find_duplicate(v));
}
