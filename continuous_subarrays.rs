use std::collections::BTreeMap;

pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {

    let mut ans: i64 = 0;
    let mut bm = BTreeMap::new();
    let mut left = 0;

    for right in 0..nums.len() {
        bm.entry(nums[right]).and_modify(|c| *c += 1).or_insert(1);

        loop {
            let first = *bm.first_key_value().unwrap().0;
            let last = *bm.last_key_value().unwrap().0;
            if (last - first).abs() <= 2 { break; }

            bm.entry(nums[left]).and_modify(|c| *c -= 1);
            if let Some(0) = bm.get(&nums[left]) {
                bm.remove(&nums[left]);
            }
            left += 1;
        }

        ans += (right - left) as i64 + 1;    
    }

    return ans;
}

#[test]
fn tests() {
    let nums = vec![5,4,2,4];
    assert_eq!(8, continuous_subarrays(nums));

    let nums = vec![1,2,3];
    assert_eq!(6, continuous_subarrays(nums));
}
