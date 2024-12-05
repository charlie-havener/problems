use std::collections::BinaryHeap;

pub fn k_big_indices(nums: Vec<i32>, k: i32) -> i32 {

    let mut ans = 0;
    let mut hp: BinaryHeap<i32> = BinaryHeap::with_capacity(k as usize + 1);
    let mut store: Vec<bool> = vec![false; nums.len()];

    for (idx,v) in nums.iter().enumerate() {
        if hp.len() == k as usize && hp.peek().unwrap() < v {
            store[idx] = true;
        }
        hp.push(*v);
        if hp.len() > k as usize {
            hp.pop();
        }
    }

    hp.clear();
    for (idx, v) in nums.iter().enumerate().rev() {
        if hp.len() == k as usize && hp.peek().unwrap() < v && store[idx] {
            ans += 1;
        }
        hp.push(*v);
        if hp.len() > k as usize {
            hp.pop();
        }
    }
    return ans;
}

#[test]
fn tests() {
    let nums = vec![2,3,6,5,2,3];
    let k = 2;
    assert_eq!(2, k_big_indcies(nums, k));

    let nums = vec![1,1,1];
    let k = 3;
    assert_eq!(0, k_big_indcies(nums, k));
}
