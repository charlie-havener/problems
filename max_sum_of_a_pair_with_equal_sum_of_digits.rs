pub fn maximum_sum(nums: Vec<i32>) -> i32 {

    let mut ans = -1;
    let mut large_sum_items: [i32;82] = [0;82]; // max digit sum would be 9*9=81

    for v in nums {
        let mut n = v as usize;
        let mut d_sum: usize = 0;
        while n > 0 {
            d_sum += n%10;
            n /= 10;
        }

        if large_sum_items[d_sum] > 0 {
            ans = ans.max(large_sum_items[d_sum] + v);
        }
        large_sum_items[d_sum] = v.max(large_sum_items[d_sum]);

    }

    return ans;
}

#[test]
fn tests() {
    let nums = vec![18,43,36,13,7];
    assert_eq!(54, maximum_sum(nums));

    let nums = vec![10,12,19,14];
    assert_eq!(-1, maximum_sum(nums));
}
