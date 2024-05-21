pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |acc, v| acc|v) << (nums.len() - 1)
}

#[test]
fn tests() {
    let v = vec![1,3];
    assert_eq!(6, subset_xor_sum(v));

    let v = vec![5,1,6];
    assert_eq!(28, subset_xor_sum(v));

    let v = vec![3,4,5,6,7,8];
    assert_eq!(480, subset_xor_sum(v));
}
