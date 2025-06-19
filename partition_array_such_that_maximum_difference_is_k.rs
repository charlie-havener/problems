pub fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {

    nums.sort_unstable();

    let mut count = 0;
    let mut target = nums[0] + k;
    for n in &nums[1..] {
        if *n > target {
            count += 1;
            target = *n + k;
        }
    }
    count += 1;
    return count;

}
