use std::collections::HashMap;

pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    let mut hm: HashMap<i32, i32> = HashMap::from([(0,-1)]);
    let mut sum = 0;
    for (idx, &n) in nums.iter().enumerate() {
        sum = (sum + n) % k;
        if let Some(&i) = hm.get(&sum) {
            if idx as i32 - i >= 2 {
                println!("{:?}", i);
                return true
            }
        }
        hm.entry(sum).or_insert(idx as i32);
    }
    return false;
}

#[test]
fn tests() {
    let n = vec![23,2,4,6,7];
    let k = 6;
    assert_eq!(true, check_subarray_sum(n,k));

    let n = vec![23,2,4,6,7];
    let k = 13;
    assert_eq!(true, check_subarray_sum(n,k));

    let n = vec![23,2,6,4,7];
    let k = 6;
    assert_eq!(true, check_subarray_sum(n,k));

    let n = vec![23,2,6,4,7];
    let k = 13;
    assert_eq!(false, check_subarray_sum(n,k));

    let n = vec![0];
    let k = 1;
    assert_eq!(false, check_subarray_sum(n,k));

    let n = vec![0,0];
    let k = 1;
    assert_eq!(true, check_subarray_sum(n,k));
}
