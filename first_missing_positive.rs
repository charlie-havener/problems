pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    for idx in 0..nums.len() {
        loop {
            if nums[idx] == idx as i32 + 1 {
                break;
            }

            if nums[idx] <= 0 || nums[idx] > nums.len() as i32 {
                nums[idx] = 0;
                break;
            }

            let i = nums[idx] as usize - 1;
            let tmp = nums[i];

            if tmp == nums[idx] {
                nums[idx] = 0;
                break;
            }

            nums[i] = nums[idx];
            nums[idx] = tmp;
        }
    }
    
    for idx in 0..nums.len() {
        if nums[idx] == 0 {
            return idx as i32 + 1;
        }
    }


    return nums.len() as i32 + 1;
}

#[test]
fn tests() {
    let v = vec![1,2,0];
    assert_eq!(3, first_missing_positive(v));

    let v = vec![3,4,-1,1];
    assert_eq!(2, first_missing_positive(v));

    let v = vec![7,8,9,11,12];
    assert_eq!(1, first_missing_positive(v));

    let v = vec![1];
    assert_eq!(2, first_missing_positive(v));

    let v = vec![0];
    assert_eq!(1, first_missing_positive(v));

    let v = vec![2];
    assert_eq!(1, first_missing_positive(v));

    let v = vec![1,2,3];
    assert_eq!(4, first_missing_positive(v));

    let v = vec![1,1,1,2,3];
    assert_eq!(4, first_missing_positive(v));
}
