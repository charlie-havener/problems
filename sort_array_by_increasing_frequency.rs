use std::collections::HashMap;

pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
    
    let l = nums.len();
    let hm = nums.into_iter().fold(HashMap::new(), |mut acc, n| {
        acc.entry(n).and_modify(|v| *v += 1).or_insert(1);
        acc
    });

    let mut ord = hm.into_iter().collect::<Vec<_>>();
    ord.sort_by_key(|k| (k.1,-k.0));
    let ans = ord.into_iter().fold(Vec::with_capacity(l), |mut acc, (k,v)| {
        for _ in 0..v {
            acc.push(k);
        }
        acc
    });

    return ans;
}

#[test]
fn test() {
    let v = vec![1,1,2,2,2,3];
    assert_eq!(vec![3,1,1,2,2,2], frequency_sort(v));

    let v = vec![2,3,1,3,2];
    assert_eq!(vec![1,3,3,2,2], frequency_sort(v));

    let v = vec![-1,1,-6,4,5,-6,1,4,1];
    assert_eq!(vec![5,-1,4,4,-6,-6,1,1,1], frequency_sort(v));
}
