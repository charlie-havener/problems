pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {

    let mut ans = i32::MIN;
    let mut reg = 0;
    let mut neg = 0;

    for n in nums {
        reg = n.max(reg + n);
        neg = (-n).max(neg - n);
        ans = ans.max(reg).max(neg);
    }

    return ans;
}

#[test]
fn tests() {
    let nums = vec![1,-3,2,3,-4];
    assert_eq!(5, max_absolute_sum(nums));

    let nums = vec![2,-5,1,-4,3,-2];
    assert_eq!(8, max_absolute_sum(nums));

    let nums = vec![1];
    assert_eq!(1, max_absolute_sum(nums));

    let nums = vec![-1];
    assert_eq!(1, max_absolute_sum(nums));

    let nums = vec![-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1];
    assert_eq!(14, max_absolute_sum(nums));

    let nums = vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1];
    assert_eq!(14, max_absolute_sum(nums));
}
