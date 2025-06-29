pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {

    const MOD: i64 = 1_000_000_007;
    let mut ans = 0;

    nums.sort_unstable();


    for (idx, val) in nums.iter().enumerate() {
        let t = target - *val;
        let i = bsearch(&nums, t);

        if i < idx as i32 { continue }
        ans = ((ans + lpow2(i - idx as i32, MOD)) as i64 % MOD) as i32;
    }

    return ans;
}

// binary search for a target value within a sorted collection
// returns the right most index less than or equal
// to target. Returns -1 if there are no elements <= target.
fn bsearch(nums: &[i32], target: i32) -> i32 {
    
    let mut left = -1;
    let mut right = nums.len() as i32;

    while right - left > 1 {
        let mid = (right - left) / 2 + left;
        if nums[mid as usize] <= target {
            left = mid;
        } else {
            right = mid;
        }
    }

    return left;
}

// fast exponentiation of 2 to the power exp
// modulo 'modulo'.
fn lpow2(exp: i32, modulo: i64) -> i32 {
    let mut base: i64 = 2;
    let mut exp: i64 = exp as i64;
    let mut ret: i64 = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            ret = (ret * base) % modulo;
        }
        base = (base * base) % modulo;
        exp >>= 1;
    }
    return ret as i32;
}

#[test]
fn tests() {
    let nums = vec![2,3,3,4,6,7];
    let target = 12;
    num_subseq(nums, target);
}
