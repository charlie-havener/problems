pub fn kth_smallest_product(nums1: Vec<i32>, nums2: Vec<i32>, k: i64) -> i64 {

    // determine the smallest and largest possible product values
    let ends = [
        nums1[0] as i64 * *nums2.last().unwrap() as i64,
        nums1[0] as i64 * nums2[0] as i64,
        *nums1.last().unwrap() as i64 * nums2[0] as i64,
        *nums1.last().unwrap() as i64 * *nums2.last().unwrap() as i64,
    ];
    let mut t_small = *ends.iter().min().unwrap();
    let mut t_large = *ends.iter().max().unwrap();

    // binary search for the kth smallest product value
    while t_large - t_small > 1 {
        let t_mid = (t_large - t_small)/2 + t_small;
        if has_k_or_more_pairs(&nums1, &nums2, t_mid, k) {
            t_large = t_mid;
        } else {
            t_small = t_mid;
        }
    }
    return t_large;
}

fn has_k_or_more_pairs(nums1: &Vec<i32>, nums2: &Vec<i32>, target: i64, k: i64) -> bool {

    // fix each num1 and bsearch nums2 for values where product is <= target
    // take into consideration the sign of fixed num1 value
    let mut total_count: i64 = 0;
    for n in nums1 {

        let mut left = -1;
        let mut right = nums2.len() as i32;

        // for negative n, the smallest products would come from taking the largest
        // values of nums2
        if *n <= 0 {
            while right - left > 1 {
                let mid = (right - left) / 2 + left;
                if (*n as i64 * nums2[mid as usize] as i64) <= target {
                    right = mid;
                } else {
                    left = mid;
                }
            }
            // left will be the left most idx with a value greater than or equal to target
            // when multiplied by n. If there are no such values, left will be -1;
            total_count += nums2.len() as i64 - right as i64;
        }

        // for positive n, the smallest products would come from taking the smallest
        // values of n2
        else if *n > 0 {
            while right - left > 1 {
                let mid = (right - left) / 2 + left;
                if (*n as i64 * nums2[mid as usize] as i64) <= target {
                    left = mid;
                } else {
                    right = mid;
                }
            }
            // right will be the left most idx with a value greater than or equal to target
            // when multiplied by n. If there are no such values, right will be the length of nums2
            total_count += (left + 1) as i64;
        }

        if total_count >= k {
            return true;
        }
    }

    return false;
}
