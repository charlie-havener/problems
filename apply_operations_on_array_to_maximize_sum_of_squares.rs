pub fn max_sum(mut nums: Vec<i32>, k: i32) -> i32 {

    const MOD: i64 = 1_000_000_007;

    let mut bit_counts = vec![0;32];

    for mut n in nums {
        let mut idx = 0;
        while n > 0 {
            if n & 1 == 1 {
                bit_counts[idx] += 1;
            }
            n >>= 1;
            idx += 1;
        }
    }

    let mut ans: i64 = 0;
    for _ in 0..k {
        let mut curr = 0;
        for idx in 0..32 {
            if bit_counts[idx] > 0 {
                curr |= 1 << idx;
                bit_counts[idx] -= 1;
            }
        }
        ans = (ans + curr * curr).rem_euclid(MOD);
    }
    
    return ans as i32;
}

#[test]
fn tests() {
    let nums = vec![2,6,5,8];
    let k = 2;
    assert_eq!(261, max_sum(nums, k));

    let nums = vec![4,5,4,7];
    let k = 3;
    assert_eq!(90, max_sum(nums, k));
}
