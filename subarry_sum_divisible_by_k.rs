use std::collections::HashMap;

pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
    let mut ans = 0;
    let mut sum = 0;
    let mut hm = HashMap::from([(0,1)]);
    
    for n in nums {
        sum = (sum + n).rem_euclid(k);
        if let Some(c) = hm.get(&sum) {
            ans += c;
        }
        hm.entry(sum).and_modify(|c| *c += 1).or_insert(1);
    }

    return ans;
}

#[test]
fn tests() {
    let v = vec![4,5,0,-2,-3,1];
    let k = 5;
    assert_eq!(7, subarrays_div_by_k(v,k));

    let v = vec![5];
    let k = 9;
    assert_eq!(0, subarrays_div_by_k(v,k));
}
