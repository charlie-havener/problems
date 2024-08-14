pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {

    nums.sort_unstable();
    
    let count_pairs = |d: i32| -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut count = 0;

        while left < nums.len() - 1 {
            if right == nums.len() {
                count += (right - 1) - left;
                left += 1;
            } else if nums[right] - nums[left] <= d {
                right += 1;
            } else {
                count += (right - 1) - left;
                left += 1;
            }
        }

        return count as i32;
    };

    let mut low = 0;
    let mut high = *nums.last().unwrap() - nums[0];

    while low < high {
        let mid = (high - low) / 2 + low;
        let count = count_pairs(mid);
        if count < k { low = mid + 1 }
        else { high = mid }
    }

    return low;
}

#[test]
fn tests() {
    let nums = vec![10,1,2,7,6,1,5];
    let k = 6;
    assert_eq!(2, smallest_distance_pair(nums, k));

    let nums = vec![10,1,2,7,6,1,5];
    let k = 19;
    assert_eq!(8, smallest_distance_pair(nums, k));

    let nums = vec![10,1,2,7,6,1,5];
    let k = 2;
    assert_eq!(1, smallest_distance_pair(nums, k));

    let nums = vec![10,1,2,7,6,1,5];
    let k = 1;
    assert_eq!(0, smallest_distance_pair(nums, k));
}
