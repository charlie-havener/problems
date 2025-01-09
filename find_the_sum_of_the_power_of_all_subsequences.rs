pub fn sum_of_power(nums: Vec<i32>, k: i32) -> i32 {

    const MOD: i32 = 1_000_000_007;

    let mut counter: [i32; 101] = [0; 101];
    counter[0] = 1;
    
    let mut inner_c: [i32; 101] = [0;101];
    for n in nums {
        inner_c.fill(0);
        for (idx, val) in counter.iter().enumerate() {
            if idx as i32 + n <= k {
                inner_c[idx + n as usize] += val;
            }
        }
        for idx in 0..101 {
            counter[idx] = ((counter[idx] << 1).rem_euclid(MOD) + inner_c[idx]).rem_euclid(MOD);
        }
    }
    return counter[k as usize];
}

#[test]
fn tests() {
    let nums = vec![1,2,3];
    let k = 3;
    assert_eq!(6, sum_of_power(nums, k));

    let nums = vec![2,3,3];
    let k = 5;
    assert_eq!(4, sum_of_power(nums, k));

    let nums = vec![1,2,3];
    let k = 7;
    assert_eq!(0, sum_of_power(nums, k));

    let nums = vec![100; 100];
    let k = 100;
    assert_eq!(818563914, sum_of_power(nums, k));
}
