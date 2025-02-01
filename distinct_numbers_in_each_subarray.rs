use std::collections::HashMap;
use std::collections::hash_map::Entry;


pub fn distinct_numbers(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut ans = Vec::with_capacity(nums.len() - k + 1);
    let mut hm = HashMap::with_capacity(k);
    for r in 0..(k-1) {
        hm.entry(&nums[r]).and_modify(|c| *c += 1).or_insert(1);
    }
    for r in (k-1)..nums.len() {
        hm.entry(&nums[r]).and_modify(|c| *c += 1).or_insert(1);
        println!("{},  {:?}", hm.len(), hm);
        ans.push(hm.len() as i32);
        if let Entry::Occupied(mut o) = hm.entry(&nums[r + 1 - k]) {
            if *o.get() == 1 {
                o.remove_entry();
            } else {
                *o.get_mut() -= 1;
            }
        }
    }
    return ans;
}

#[test]
fn tests() {
    //let nums = vec![1,2,3,2,2,1,3];
    //let k = 3;
    //assert_eq!(vec![3,2,2,2,3], distinct_numbers(nums, k));

    let nums = vec![11,8,9,12,46,42,34,20,42,6,34,50,6,36,49,34,43,50,46];
    let k = 4;
    assert_eq!(vec![4,4,4,4,4,3,4,4,4,3,4,4,4,4,4,4], distinct_numbers(nums, k));

}
