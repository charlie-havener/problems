pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut p = 1;
    let mut prod = vec![1; nums.len()];

    for idx in 0..nums.len() - 1 {
        p *= nums[idx];
        prod[idx] = p;
    }
    prod[nums.len() - 1] = p;
    p = *nums.last().unwrap();
    for idx in (1..(nums.len()-1)).rev() {
        prod[idx] = p * prod[idx - 1];
        p *= nums[idx];
    }
    prod[0] = p;
    return prod;
}

#[test]
fn test() {
    let v = vec![1,2,3,4];
    let ans = vec![24,12,8,6];
    assert_eq!(ans, product_except_self(v));
    let v = vec![-1,1,0,-3,3];
    let ans = vec![0,0,9,0,0];
    assert_eq!(ans, product_except_self(v));
    let v = vec![1,1];
    let ans = vec![1,1];
    assert_eq!(ans, product_except_self(v));
    let v = vec![0,9,0];
    let ans = vec![0,0,0];
    assert_eq!(ans, product_except_self(v));
}
