use std::collections::HashMap;

pub fn tuple_same_product(nums: Vec<i32>) -> i32 {

    let mut freq: HashMap<i32, i32> = HashMap::new();
    for i in 0..(nums.len()-1) {
        for j in (i+1)..nums.len() {
            freq.entry(nums[i] * nums[j]).and_modify(|f| *f += 1).or_insert(1);
        }
    }

    let mut ans = 0;
    for v in freq.values() {
        let combinations = *v * (*v-1) / 2;
        ans += combinations * 8;
    }

    return ans;
}
