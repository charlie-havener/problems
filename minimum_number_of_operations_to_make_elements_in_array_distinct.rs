pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    let mut present = [false; 101];
    for (idx, num) in nums.iter().enumerate().rev() {
        if present[*num as usize] {
            return idx as i32 / 3 + 1;
        }
        present[*num as usize] = true;
    }
    return 0;
}
