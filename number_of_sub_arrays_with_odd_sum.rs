pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {

    const MOD: i32 = 1_000_000_007;

    let mut even_count: i32 = 1;
    let mut odd_count: i32 = 0;
    let mut sum: i32 = 0;
    let mut ans: i32 = 0;

    for num in arr {
        sum = (sum + num) & 1;
        match sum.rem_euclid(2) == 0 {
            true => {
                ans = (ans + odd_count).rem_euclid(MOD);
                even_count += 1;
            }
            false => {
                ans = (ans + even_count).rem_euclid(MOD);
                odd_count += 1;
            }
        }
    }

    return ans;
}

#[test]
fn tests() {
    let arr = vec![1,3,5];
    assert_eq!(4, num_of_subarrays(arr));

    let arr = vec![2,4,6];
    assert_eq!(0, num_of_subarrays(arr));

    let arr = vec![1,2,3,4,5,6,7];
    assert_eq!(16, num_of_subarrays(arr));
}
