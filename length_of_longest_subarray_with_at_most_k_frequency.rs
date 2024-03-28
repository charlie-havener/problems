use std::collections::HashMap;

pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut left = 0;
    let mut right = 0;
    let mut ans = 0;
    let mut hm: HashMap<i32, i32> = HashMap::new();

    while right < nums.len() {
        hm.entry(nums[right]).and_modify(|c| *c += 1).or_insert(1);
        if hm.get(&nums[right]).unwrap().cmp(&k) == std::cmp::Ordering::Greater {
            while nums[right] != nums[left] {
                hm.entry(nums[left]).and_modify(|c| *c -= 1);
                left += 1;
            }
            hm.entry(nums[left]).and_modify(|c| *c -= 1);
            left += 1;
        }
        ans = ans.max(right - left + 1);
        right += 1;
    }
    return ans as i32;
}

#[test]
fn test() {
    let v = vec![1,2,3,1,2,3,1,2];
    let k = 2;
    assert_eq!(6, max_subarray_length(v,k));

    let v = vec![1,2,1,2,1,2,1,2];
    let k = 1;
    assert_eq!(2, max_subarray_length(v,k));

    let v = vec![5,5,5,5,5,5,5];
    let k = 4;
    assert_eq!(4, max_subarray_length(v,k));

    let v = vec![1,2,3,1,2,3,1,2];
    let k = 3;
    assert_eq!(8, max_subarray_length(v,k));

    let v = vec![5,5,5,5,5,5,5];
    let k = 1;
    assert_eq!(1, max_subarray_length(v,k));
    
    let v = vec![3,1,1];
    let k = 1;
    assert_eq!(2, max_subarray_length(v,k));


}
