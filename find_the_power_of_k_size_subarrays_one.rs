pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {

    if k == 1 {
        return nums;
    }

    let k = k as usize;
    let mut ans = Vec::with_capacity(nums.len() - k);

    let (mut left, mut right) = (0,0);
    
    while left < nums.len() - k + 1 {
        
        // valid sequence found
        if right - left == k - 1 {
            ans.push(nums[right]);
            left += 1;
        }
        else {
            right += 1;
            if nums[right-1]+1 != nums[right] {
                while left < right && left < nums.len() - k + 1 {
                    ans.push(-1);
                    left += 1;
                }
            }

        }

    }

    return ans;
}

#[test]
fn tests() {
    let nums = vec![1,2,3,4,3,2,5];
    let k = 3;
    assert_eq!(vec![3,4,-1,-1,-1], results_array(nums, k));

    let nums = vec![1,2,3,4,3,2,5];
    let k = 2;
    assert_eq!(vec![2,3,4,-1,-1,-1], results_array(nums, k));

    let nums = vec![1,2,3,4,3,2,5];
    let k = 1;
    assert_eq!(vec![1,2,3,4,3,2,5], results_array(nums, k));

    let nums = vec![1,2,3];
    let k = 3;
    assert_eq!(vec![3], results_array(nums, k));

    let nums = vec![2,2,2,2,2];
    let k = 4;
    assert_eq!(vec![-1,-1], results_array(nums, k));

    let nums = vec![3,2,3,2,3,2];
    let k = 2;
    assert_eq!(vec![-1,3,-1,3,-1], results_array(nums, k));

    let nums = vec![1,2,3,2];
    let k = 3;
    assert_eq!(vec![3,-1], results_array(nums, k));
}
