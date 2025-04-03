pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let mut i: i64 = 0;
    let mut d: i64 = 0;
    let mut a: i64 = 0;
    for n in nums {
        a = a.max(d * n as i64);
        d = d.max(i - n as i64);
        i = i.max(n as i64);
    }
    return a;
}

#[test]
fn tests() {
    let nums = vec![12,6,1,2,7];
    assert_eq!(77, maximum_triplet_value(nums));

    let nums = vec![1,10,3,4,19];
    assert_eq!(133, maximum_triplet_value(nums));
    
    let nums = vec![1,2,3];
    assert_eq!(0, maximum_triplet_value(nums));
}
