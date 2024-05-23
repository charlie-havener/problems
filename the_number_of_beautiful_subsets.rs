use std::collections::HashSet;

pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
    
    fn recurse(idx: usize, nums: &Vec<i32>, k: i32, hs: &mut HashSet<i32>) -> i32 {

        if idx >= nums.len() { 
            return if hs.is_empty() { 0 } else { 1 }
        }

        let num = nums[idx];

        // path if it wasn't added
        let mut ans = recurse(idx + 1, nums, k, hs);

        // if adding num would still be a valid set
        let mut added = false;
        if !hs.contains(&(num - k)) && !hs.contains(&(num + k)) {
            added = hs.insert(num);
            ans += recurse(idx + 1, nums, k, hs);
        }
        if added {
            hs.remove(&num);
        }

        return ans;
    }


    let mut hs: HashSet<i32> = HashSet::new();
    return recurse(0, &nums, k, &mut hs)
}

#[test]
fn tests() {
    let nums = vec![2,4,6];
    let k = 2;
    assert_eq!(4, beautiful_subsets(nums, k));

    let nums = vec![1];
    let k = 1;
    assert_eq!(1, beautiful_subsets(nums, k));

    let nums = vec![1,1,1,1];
    let k = 0;
    assert_eq!(4, beautiful_subsets(nums, k));

    let nums = vec![1,2,3,4];
    let k = 10;
    assert_eq!(15, beautiful_subsets(nums, k));
}
