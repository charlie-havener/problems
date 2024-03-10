use std::collections::HashMap;

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut n1 = &nums1;
    let mut n2 = &nums2;
    if n1.len() > n2.len() {
        std::mem::swap(&mut n1,&mut n2);
    }

    let mut ans = Vec::new();
    
    let mut counts = n1.iter().fold(vec![0;1001], |mut acc, n| {
        acc[*n as usize] += 1;
        acc
    });
    for &n in n2 {
        if counts[n as usize] > 0 {
            counts[n as usize] = 0;
            ans.push(n);
        }
    }

    return ans;
}

#[test]
fn test() {
    let nums1 = vec![1,2,2,1];
    let nums2 = vec![2,2];
    assert_eq!(vec![2], intersection(nums1, nums2));
    
    let nums1 = vec![4,9,5];
    let nums2 = vec![9,4,9,8,4];
    assert_eq!(vec![9,4], intersection(nums1, nums2));

    let nums1 = vec![1];
    let nums2 = vec![2];
    assert_eq!(Vec::<i32>::new(), intersection(nums1, nums2));
    
}
