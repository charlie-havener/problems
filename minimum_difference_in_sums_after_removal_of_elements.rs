use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn minimum_difference(nums: Vec<i32>) -> i64 {

    let mut ans: i64 = i64::MAX;
    let l = nums.len();
    let k = nums.len() / 3;

    // precompute the right values, storing the min values
    let mut lefts = Vec::with_capacity(k);
    let mut sum: i64 = 0;
    let mut bh: BinaryHeap<i32> = BinaryHeap::with_capacity(k);

    // get the sum of the initial n elements
    // this is the value with 0 removals
    for i in 0..k {
        bh.push(nums[i]);
        sum += nums[i] as i64;
    }
    lefts.push(sum);

    // want the minimum n elements from the first 2n elements
    // step through one at a time and store the lowest vals in lefts
    for i in 0..k {
        let t = bh.pop().unwrap();
        sum -= t as i64;
        sum += nums[i + k] as i64;
        lefts.push((*lefts.last().unwrap()).min(sum));
    }


    // now we want the max sum of the rightmost elements.
    // if we have r removals on the right, then there can be
    // at most k-r removals on the left, so lookup that value
    // the answer is the min difference
    sum = 0;
    let mut bh: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(k);
    for i in 0..k {
        bh.push(Reverse(nums[l-i-1]));
        sum += nums[l-i-1] as i64;
    }
    ans = ans.min(lefts[k] - sum);

    for i in 0..k {
        let Reverse(t) = bh.pop().unwrap();
        sum -= t as i64;
        sum += nums[l-i-k-1] as i64;
        ans = ans.min(lefts[k-i-1] - sum);
    }

    return ans;
}
