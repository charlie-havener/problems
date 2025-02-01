pub fn is_array_special(nums: Vec<i32>) -> bool {

    for pair in nums.windows(2) {
        let a = pair[0] % 2;
        let b = pair[1] % 2;
        if a^b == 1 {
            continue;
        }
        return false;
    }
    return true;

}
