pub fn xor_beauty(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |acc, v| acc^v)
}

#[test]
fn tests() {
    let v = vec![1,4];
    assert_eq!(5, xor_beauty(v));

    let v = vec![15,45,20,2,34,35,5,44,32,30];
    assert_eq!(34, xor_beauty(v));
}
