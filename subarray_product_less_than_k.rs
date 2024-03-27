pub fn num_subarray_product_less_than_k(mut nums: Vec<i32>, k: i32) -> i32 {
    let mut left = 0;
    let mut right = 0;
    let mut product = 1;
    let mut ans = 0;

    if k == 0 {
        return 0;
    }

    loop {
        //println!("{left}, {right}, {product}");
        if left >= nums.len() { return ans };

        while right <= nums.len() {
            if right == nums.len() {
                break;
            } else {
                product *= nums[right];
                if product >= k {
                    break;
                }
                right += 1;
            }
        }

        ans += (right - left) as i32;
        left += 1;
        right = left;
        product = 1;
    }
}

#[test]
fn test() {
    let v = vec![10,5,2,6];
    let k = 100;
    assert_eq!(8, num_subarray_product_less_than_k(v, k));

    let v = vec![1,2,3];
    let k = 0;
    assert_eq!(0, num_subarray_product_less_than_k(v, k));


}
