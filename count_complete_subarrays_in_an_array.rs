use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;

pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {

    let mut left = 0;
    let mut right = 0;
    let mut cnt = HashMap::new();
    let mut ans = 0;
    let distinct = nums.iter().collect::<HashSet<_>>().len();

    while right < nums.len() {

        cnt.entry(nums[right]).and_modify(|v| *v += 1).or_insert(1);

        while cnt.len() >= distinct {
            ans += nums.len() - right;
            match cnt.entry(nums[left]) {
                Entry::Occupied(mut o) => {
                    if *o.get() == 1 {
                        o.remove_entry();
                    } else {
                        *o.get_mut() -= 1;
                    }
                },
                _ => (),
            }
            left += 1;
        }

        right += 1;

    }

    return ans as i32;
}

#[test]
fn tests() {
    let nums = vec![1,3,1,2,2];
    assert_eq!(4, count_complete_subarrays(nums));

    let nums = vec![5,5,5,5];
    assert_eq!(10, count_complete_subarrays(nums));

    let nums = vec![1,2,3,4,5];
    assert_eq!(1, count_complete_subarrays(nums));
}
