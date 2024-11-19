use std::collections::HashMap;

pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {

    let k = k as usize;

    let mut ans = 0_i64;
    let mut sum = 0_i64;
    let mut hm: HashMap<i32, usize> = HashMap::with_capacity(k as usize);

    // get the initial setup
    for i in 0..k {
        hm.entry(nums[i]).and_modify(|count| *count += 1).or_insert(1);
        sum += nums[i] as i64;
    }
    if hm.len() == k {
        ans = ans.max(sum);
    }
    

    // slide through the remainder
    for i in k..nums.len() {
        let tail = i - k;
        hm.entry(nums[tail]).and_modify(|count| *count -= 1);
        if hm.get(&nums[tail]) == Some(&0) {
            hm.remove(&nums[tail]);
        }
        sum -= nums[tail] as i64;

        hm.entry(nums[i]).and_modify(|count| * count += 1).or_insert(1);
        sum += nums[i] as i64;

        if hm.len() ==k {
            ans = ans.max(sum);
        }
    }
    return ans;
}

#[test]
fn tests() {
    let nums = vec![1,5,4,2,9,9,9];
    let k = 3;
    assert_eq!(15, maximum_subarray_sum(nums, k));

    let nums = vec![4,4,4];
    let k = 3;
    assert_eq!(0, maximum_subarray_sum(nums, k));
}
