pub fn find_lhs(mut nums: Vec<i32>) -> i32 {

    nums.sort_unstable();

    let mut ans = 0;
    let mut left = 0;
    for right in 0..nums.len() {
        while nums[right] - nums[left] > 1 {
            left += 1;
        }
        if nums[right] - nums[left] == 1 {
            ans = ans.max(right - left + 1);
        }
    }

    return ans as i32;
}


/*use std::collections::BTreeMap;

pub fn find_lhs(nums: Vec<i32>) -> i32 {

    let mut store = BTreeMap::new();
    let mut ans = 0;
    for n in &nums {
        store.entry(*n).and_modify(|c| *c += 1).or_insert(1);
    }
    let mut it = store.into_iter();
    // unwrap is safe since nums.len() >= 1
    // therefore store must initially always have at least one element;
    let mut prev = it.next().unwrap();

    while let Some((val, cnt)) = it.next() {
        if prev.0 + 1 == val {
            ans = ans.max(prev.1 + cnt);
        }
        prev = (val, cnt);
    }


    return ans;

}
*/
