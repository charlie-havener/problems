use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
    
    let mut bh: BinaryHeap<(Reverse<i32>, Reverse<usize>)> = nums
        .iter()
        .enumerate()
        .fold(BinaryHeap::with_capacity(nums.len()), |mut acc, (idx, val)| {
            acc.push((Reverse(*val), Reverse(idx)));
            acc
    });

    for _ in 0..k {
        let (Reverse(v), Reverse(idx)) = bh.pop().unwrap();
        let n = v * multiplier;
        nums[idx] = n;
        bh.push((Reverse(n), Reverse(idx)));
    }

    return nums;
}

#[test]
fn tests() {
    let nums = vec![2,1,3,5,6];
    let k = 5;
    let multiplier = 2;
    assert_eq!(vec![8,4,6,5,6], get_final_state(nums, k, multiplier));

    let nums = vec!    [1,2];
    let k = 3;
    let multiplier = 4;
    assert_eq!(vec![16,8], get_final_state(nums, k, multiplier));
}
