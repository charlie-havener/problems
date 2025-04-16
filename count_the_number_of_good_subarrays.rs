use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {

    let mut hm = HashMap::new();
    let mut left = 0;
    let mut right = 0;
    let mut count = 0;
    let mut ans = 0_i64;

    while right < nums.len() {

        match hm.entry(nums[right]) {
            Entry::Occupied(mut e) => {
                count += e.get();
                *e.get_mut() += 1;
            },
            Entry::Vacant(e) => {
                e.insert(1);
            }
        }

        // don't need to check that left < right since you wouldn't have any pairs
        // when you get that far. k >= 1 (can't be 0)
        while count >= k {
            ans += (nums.len() - right) as i64;
            // entry will always be occupied
            if let Entry::Occupied(mut e) = hm.entry(nums[left]) {
                count -= e.get() - 1;
                if &1 == e.get() {
                    e.remove();
                } else {
                    *e.get_mut() -= 1;
                }
            } else {
                panic!("wtf");
            }
            left += 1;
        }

        right += 1;
    }
    
    return ans;
}

#[test]
fn tests() {
    let nums = vec![1,1,1,1,1];
    let k = 10;
    assert_eq!(1, count_good(nums, k));

    let nums = vec![3,1,4,3,2,2,4];
    let k = 2;
    assert_eq!(4, count_good(nums, k));
}
