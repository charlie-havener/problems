pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {

    let mut s: Vec<i32> = Vec::with_capacity(nums.len());
    s.push(-1);
    for idx in 1..nums.len() {
        // if the pairs parity is different one must be equal to 0 while the other is 1
        if (nums[idx] % 2) + (nums[idx-1] % 2) == 1 {
            s.push(s[idx-1]);
        } else {
            s.push(idx as i32);
        }
    }

    let mut ans = Vec::with_capacity(queries.len());
    for q in queries {
        if s[q[1] as usize] <= q[0] { ans.push(true) }
        else { ans.push(false) }
    }


    return ans;
}

#[test]
fn tests() {
    let nums = vec![3,4,1,2,6];
    let queries = vec![vec![0,4]];
    assert_eq!(vec![false], is_array_special(nums, queries));

    let nums = vec![4,3,1,6];
    let queries = vec![vec![0,2],vec![2,3]];
    assert_eq!(vec![false, true], is_array_special(nums, queries));
}
