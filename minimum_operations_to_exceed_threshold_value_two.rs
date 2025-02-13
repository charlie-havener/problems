use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {

    let mut ans = 0;
    let hk = k / 2;

    let mut bh: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    for n in nums {
        if n < k {
            bh.push(Reverse(n));
        }
    }

    while bh.len() > 1 {
        ans += 1;
        let Reverse(a) = bh.pop().unwrap();
        let Reverse(b) = bh.pop().unwrap();
        if a >= hk { continue }
        let c = a * 2 + b;
        if c < k {
            bh.push(Reverse(c));
        }
    }

    if bh.len() == 1 {
        ans += 1;
    }


    return ans;
}
