pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut ptr = 0;
    let mut left = 0;
    let mut right = nums.len() - 1;

    while ptr <= right {
        if nums[ptr] == 0 {
            nums[ptr] = nums[left];
            nums[left] = 0;
            left += 1;
            ptr += 1;
        }
        else if nums[ptr] == 2 {
            nums[ptr] = nums[right];
            nums[right] = 2;
            if right == 0 {
                return;
            }
            right -= 1;
        } else {
            ptr += 1
        }
    }
}

#[test]
fn tests() {
    let mut v = vec![2,0,2,1,1,0];
    sort_colors(&mut v);
    assert_eq!(vec![0,0,1,1,2,2], v);

    let mut v = vec![2,0,1];
    sort_colors(&mut v);
    assert_eq!(vec![0,1,2], v);

    let mut v = vec![0,0,0,0,0];
    sort_colors(&mut v);
    assert_eq!(vec![0,0,0,0,0], v);

    let mut v = vec![0,0,0,0,0];
    sort_colors(&mut v);
    assert_eq!(vec![0,0,0,0,0], v);

    let mut v = vec![2,2,2,2,2];
    sort_colors(&mut v);
    assert_eq!(vec![2,2,2,2,2], v);
}
