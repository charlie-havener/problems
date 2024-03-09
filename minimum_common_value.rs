pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {

    let mut ptr1 = 0;
    let mut ptr2 = 0;

    loop {
        if ptr1 == nums1.len() || ptr2 == nums2.len() {
            return -1;
        } else if nums1[ptr1] == nums2[ptr2] {
            return nums1[ptr1];
        } else if nums1[ptr1] < nums2[ptr2] {
            ptr1 += 1;
        } else {
            ptr2 += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, get_common(vec![1,2,3], vec![2,4]));
        assert_eq!(2, get_common(vec![1,2,3,6], vec![2,3,4,5]));
        assert_eq!(-1, get_common(vec![1,2,3], vec![4,5,6]));
    }
}
