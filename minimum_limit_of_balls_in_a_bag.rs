pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
    
    let mut ans = i32::MAX;
    let mut left = 1;
    let mut right = *nums.iter().max().unwrap();

    while left <= right {
        let mid = left + (right - left) / 2;
        
        let mut ops = max_operations;
        for i in 0..nums.len() {
            if ops < 0 { break }
            let curr = nums[i];
            if curr <= mid { continue }
            else {
                ops -= (curr - 1) / mid;
            }
        }
        
        if ops >= 0 {
            right = mid - 1;
            ans = ans.min(mid);
        }
        else { left = mid + 1 }
    }

    return ans;
}

#[test]
fn tests() {
    let nums = vec![9];
    let max_operations = 2;
    assert_eq!(3, minimum_size(nums, max_operations));

    let nums = vec![2,4,8,2];
    let max_operations = 4;
    assert_eq!(2, minimum_size(nums, max_operations));
}
