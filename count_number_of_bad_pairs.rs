use std::collections::{HashMap, hash_map::Entry};

pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
    
    let mut hm: HashMap<i32, usize> = HashMap::new();
    let mut ans: i64 = 0;

    for (idx, val) in nums.iter().enumerate() {
        let d = idx as i32 - *val;
        ans += idx as i64;
        let ent = hm.entry(d); // first hash
        if let Entry::Occupied(ref o) = ent {
            ans -= *o.get() as i64;
        }
        ent.and_modify(|v| *v += 1).or_insert(1);
    }

    return ans;
}
