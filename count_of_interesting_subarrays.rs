use std::collections::HashMap;

pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {

    let mut ans: i64 = 0;
    let mut hm: HashMap<i32, i32> = HashMap::new();
    hm.insert(0,1);

    let mut rolling = 0;
    for n in nums {
        
        let t = n % modulo;
        if t == k {
            rolling += 1;
            if rolling >= modulo {
                rolling -= modulo;
            }
        }

        let needed = (rolling - k).rem_euclid(modulo);
        ans += *hm.get(&needed).unwrap_or(&0) as i64;
        hm.entry(rolling).and_modify(|c| *c += 1).or_insert(1);
    }

    return ans;
}

#[test]
fn tests() {
    let nums = vec![3,2,4];
    let modulo = 2;
    let k = 1;
    assert_eq!(3, count_interesting_subarrays(nums, modulo, k));

    let nums = vec![3,1,9,6];
    let modulo = 3;
    let k = 0;
    assert_eq!(2, count_interesting_subarrays(nums, modulo, k));

    let nums = vec![1,1,1,1,1,1];
    let modulo = 2;
    let k = 1;
    assert_eq!(12, count_interesting_subarrays(nums, modulo, k));

    let nums = vec![1,1,1,1,1,1,1,1,1,1,1,1];
    let modulo = 2;
    let k = 1;
    assert_eq!(42, count_interesting_subarrays(nums, modulo, k));

    let nums = vec![1,2,3,4,5,6,7,8,9,10,1,2,3,4,5,6,7,8,9,10];
    let modulo = 4;
    let k = 3;
    assert_eq!(28, count_interesting_subarrays(nums, modulo, k));
}
