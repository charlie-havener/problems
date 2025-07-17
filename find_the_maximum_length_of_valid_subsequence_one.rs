pub fn maximum_length(nums: Vec<i32>) -> i32 {

    let mut e = 0;
    let mut o = 0;
    let mut a = 0;

    let mut f = (nums[0]&1)^1;
    for n in nums {
        if n&1 == 1 { o += 1 }
        else { e += 1 }
        if f^(n&1) == 1 {
            a += 1;
            f ^= 1;
        }
    }

    return e.max(o).max(a);
}
