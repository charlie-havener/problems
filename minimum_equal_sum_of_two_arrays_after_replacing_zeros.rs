pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {

    let (mut s1, mut z1) = (0_i64, 0_i64);
    for n in nums1 {
        if n == 0 { z1 += 1 }
        s1 += n as i64;
    }

    let (mut s2, mut z2) = (0_i64, 0_i64);
    for n in nums2 {
        if n == 0 { z2 += 1 }
        s2 += n as i64;
    }

    let target = (s1+z1).max(s2+z2);

    if (s1 + z1) < target && z1 == 0 { return -1 }
    if (s2 + z2) < target && z2 == 0 { return -1 }
    
    return target;
}
