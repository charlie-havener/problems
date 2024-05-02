use std::collections::HashSet;

pub fn find_max_k(nums: Vec<i32>) -> i32 {
    let mut hs = HashSet::new();
    nums.into_iter().fold(-1, |mut acc, n| {
        match hs.get(&(n*-1)) {
            Some(_) => { acc = acc.max(n.abs()); },
            None => { hs.insert(n); }
        }
        acc
    })
}

#[test]
fn tests() {
    let nums = vec![-1,2,-3,3];
    assert_eq!(3, find_max_k(nums));

    let nums = vec![-1,10,6,7,-7,1];
    assert_eq!(7, find_max_k(nums));

    let nums = vec![-10,8,6,7,-2,-3];
    assert_eq!(-1, find_max_k(nums));
}
