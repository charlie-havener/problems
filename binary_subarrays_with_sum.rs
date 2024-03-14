use std::collections::HashMap;

pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
    fn helper(nums: &Vec<i32>, goal: i32) -> i32 {
        let mut start = 0;
        let mut current_sum = 0;
        let mut total_count = 0;

        for end in 0..nums.len() {
            current_sum += nums[end];

            while start <= end && current_sum > goal {
                current_sum -= nums[start];
                start += 1;
            }

            total_count += end + 1 - start;
        }

        return total_count as i32;
    }

    return helper(&nums, goal) - helper(&nums, goal-1);
}

#[test]
fn test() {
    let v = vec![1,0,1,0,1];
    let n = 2;
    assert_eq!(4, num_subarrays_with_sum(v,n));
    let v = vec![0,0,0,0,0];
    let n = 0;
    assert_eq!(15, num_subarrays_with_sum(v,n));
    let v = vec![1];
    let n = 1;
    assert_eq!(1, num_subarrays_with_sum(v,n));
}
