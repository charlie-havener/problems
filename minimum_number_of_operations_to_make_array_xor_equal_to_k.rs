pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let curr = nums.into_iter().fold(0, |acc, n| acc^n);
    let mut ans = 0;
    for bit in 0..32 {
        let a = (curr >> bit) & 1;
        let b = (k >> bit) & 1;
        if a != b {ans += 1}
    }
    return ans;
}

#[test]
fn tests() {
    let v = vec![2,1,3,4];
    let k = 1;
    assert_eq!(2, min_operations(v,k));

    let v = vec![2,0,2,0];
    let k = 0;
    assert_eq!(0, min_operations(v,k));
}
