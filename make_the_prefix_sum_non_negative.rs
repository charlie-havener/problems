use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn make_pref_sum_non_negative(nums: Vec<i32>) -> i32 {

    let mut sum: i64 = 0;
    let mut bh: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut swaps = 0;

    for n in nums {
        if n < 0 {
            bh.push(Reverse(n));
        }
        sum += n as i64;
        if sum < 0 {
            sum -= bh.pop().unwrap().0 as i64;
            swaps += 1;
        }
    }

    return swaps;
}
