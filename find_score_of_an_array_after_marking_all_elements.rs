use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn find_score(mut nums: Vec<i32>) -> i64 {
    
    let mut score: i64 = 0;
    let mut bh: BinaryHeap<(Reverse<i32>, Reverse<i32>)> = BinaryHeap::new();
    for (idx, val) in nums.iter().enumerate() {
        bh.push((Reverse(*val), Reverse(idx as i32)));
    }
    while let Some((Reverse(n), Reverse(idx))) = bh.pop() {
        if nums[idx as usize] != -1 {
            score += n as i64;
            nums[idx as usize] = -1;
            if idx-1 >= 0 { nums[idx as usize - 1] = -1; }
            if idx as usize + 1 < nums.len() { nums[idx as usize + 1] = -1 }
        }
    }

    return score;
}

#[test]
fn tests() {
    let nums = vec![2,1,3,4,5,2];
    assert_eq!(7, find_score(nums));

    let nums = vec![2,3,5,1,3,2];
    assert_eq!(5, find_score(nums));
}
