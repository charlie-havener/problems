use std::cmp::Reverse;

pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut nums: Vec<(usize, i32)> = nums.into_iter().enumerate().fold(Vec::new(), | mut acc, i| {
        acc.push(i);
        acc
    });
    nums.sort_unstable_by_key(|k| Reverse(k.1));
    nums.truncate(k as usize);
    nums.sort_by_key(|k| k.0);
    return nums.iter().map(|v| v.1).collect();
}
