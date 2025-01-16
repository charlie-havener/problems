pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {

    let mut ans = 0;
    if nums1.len() % 2 == 1 {
        for n in &nums2 { ans ^= n }
    }
    if nums2.len() % 2 == 1 {
        for n in &nums1 { ans ^= n }
    }

    return ans;
}

#[test]
fn tests() {
    let nums1 = vec![2,1,3];
    let nums2 = vec![10,2,5,0];
    assert_eq!(13, xor_all_nums(nums1, nums2));

    let nums1 = vec![1,2];
    let nums2 = vec![3,4];
    assert_eq!(0, xor_all_nums(nums1, nums2));
}
