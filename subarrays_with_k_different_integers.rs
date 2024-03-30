use std::collections::HashMap;

pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
    
    fn helper(nums: &Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut hm: HashMap<i32, i32> = HashMap::new();
        let mut left = 0;
        let k = k as usize;

        for right in 0..nums.len() {
            hm.entry(nums[right]).and_modify(|c| *c += 1).or_insert(1);

            while hm.len() > k {
                match hm.get_mut(&nums[left]) {
                    Some(1) => {hm.remove(&nums[left]);},
                    Some(v) => *v -= 1,
                    _ => (),
                };
                left += 1;
            }
            ans += right + 1 - left;
        }
        return ans as i32;
    }    
    return helper(&nums, k) - helper(&nums, k-1);
}

#[test]
fn tests() {
    let v = vec![1,2,1,2,3];
    let k = 2;
    assert_eq!(7, subarrays_with_k_distinct(v,k));

    let v = vec![1,2,1,3,4];
    let k = 3;
    assert_eq!(3, subarrays_with_k_distinct(v,k));
}
