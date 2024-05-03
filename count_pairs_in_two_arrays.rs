use std::cmp::Ordering;

pub fn count_pairs(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
    let mut ans = 0;
    let n = nums1.len();
    let mut d = (0..n).fold(Vec::with_capacity(n), |mut acc, v| {
        acc.push(nums1[v] - nums2[v]);
        acc
    });
    d.sort();

    let mut left = 0;
    let mut right = n - 1;
    while left < right {
        if d[left] + d[right] > 0 {
            ans += (right - left) as i64;
            right -= 1;
        } else {
            left += 1;
        }
    }

    return ans;

}

#[test]
fn tests() {
    let nums1 = vec![2,1,2,1];
    let nums2 = vec![1,2,1,2];
    assert_eq!(1, count_pairs(nums1, nums2));

    let nums1 = vec![1,10,6,2];
    let nums2 = vec![1,4,1,5];
    assert_eq!(5, count_pairs(nums1, nums2));

    let nums1 = vec![5,1,1,15,3,14,19,1,9,12,6,8,2,4,19,17,19,5];
    let nums2 = vec![1,16,5,3,7,9,19,3,7,2,13,4,4,17,13,12,19,16];
    assert_eq!(71, count_pairs(nums1, nums2));
}
