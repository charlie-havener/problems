use std::collections::HashMap;
pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
    

    if nums1.len() > nums2.len() {
        std::mem::swap(&mut nums1, &mut nums2);
    }

    let mut hm = nums1.iter().fold(HashMap::new(), |mut acc, v| {
        acc.entry(*v).and_modify(|c| *c += 1).or_insert(1);
        acc
    });

    let mut ans = vec![];
    for n in nums2 {
        if let Some(c) = hm.get_mut(&n) {
            if *c > 0 {
                ans.push(n);
                *c -= 1;
            }
        }
    }

    return ans;

}

#[test]
fn tests() {
    let n1 = vec![1,2,2,1];
    let n2 = vec![2,2];
    assert_eq!(vec![2,2], intersect(n1, n2));

    let n1 = vec![4,9,5];
    let n2 = vec![9,4,9,8,4];
    assert_eq!(vec![9,4], intersect(n1, n2));
}
