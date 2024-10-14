use std::collections::BinaryHeap;

pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {

    let mut ans: i64 = 0;
    let mut bh = BinaryHeap::from(nums);

    for _ in 0..k {
        let t = bh.pop().unwrap();
        ans += t as i64;
        bh.push((t+2)/3);
    }
    return ans;
}

#[test]
fn tests() {
    let nums = vec![10,10,10,10,10];
    let k = 5;
    assert_eq!(50, max_kelements(nums, k));

    let nums = vec![1,10,3,3,3];
    let k = 3;
    assert_eq!(17, max_kelements(nums, k));
}

