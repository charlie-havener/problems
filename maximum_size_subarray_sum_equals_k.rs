use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
    
    let mut ans = 0;
    let mut prefix_sum = 0;
    let mut hs = HashMap::from([(0,-1)]);

    for (idx, val) in nums.iter().enumerate() {
        prefix_sum += val;
        let needed = prefix_sum - k;
        if let Entry::Occupied(o) = hs.entry(needed) {
            ans = ans.max(idx as i32 - *o.get());
        }
        if let Entry::Vacant(o) = hs.entry(prefix_sum) {
            o.insert(idx as i32);
        }
    }
    return ans;
}
