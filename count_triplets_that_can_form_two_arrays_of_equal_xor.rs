
use std::collections::HashMap;

pub fn count_triplets(arr: Vec<i32>) -> i32 {
    let mut ans = 0;

    let mut seen_prefixs = HashMap::from([(0,1)]);
    let mut idx_sum = HashMap::from([(0,0)]);
    
    let mut total = 0;
    for idx in 0..arr.len() {
        total ^= arr[idx];

        if let Some(count) = seen_prefixs.get(&total) {
            ans += idx * count - idx_sum.get(&total).unwrap();
        }

        seen_prefixs.entry(total).and_modify(|v| *v += 1).or_insert(1);
        idx_sum.entry(total).and_modify(|v| *v += idx + 1).or_insert(idx + 1);

    }

    return ans as i32;
}

#[test]
fn tests() {
    let arr = vec![2,3,1,6,7];
    assert_eq!(4, count_triplets(arr));

    let arr = vec![1,1,1,1,1];
    assert_eq!(10, count_triplets(arr));


}
