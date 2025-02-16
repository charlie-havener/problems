
fn backtrack(used: &mut Vec<bool>, ans: &mut Vec<i32>, nums: &Vec<i32>, idx: usize) -> bool {
    if idx >= ans.len() { 
        return true;
    }

    if ans[idx] != 0 {
        return backtrack(used, ans, nums, idx + 1);
    }

    for n in nums {

        // check that the number can be addded
        if used[*n as usize] { continue }
        if *n != 1 && (ans.len() <= idx + *n as usize || ans[idx + *n as usize] != 0) { continue }

        // add the number
        ans[idx] = *n;
        if *n != 1 { ans[idx + *n as usize] = *n; }
        used[*n as usize] = true;

        // continue, and return true when an answer is found
        if backtrack(used, ans, nums, idx + 1) {
            return true;
        }

        // remove the number if search was unsuccessful
        ans[idx] = 0;
        if *n != 1 { ans[idx + *n as usize] = 0; }
        used[*n as usize] = false;
    }
    
    return false;
}


pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {

    let mut ans = vec![0; n as usize * 2 - 1];

    let nums = (1..=n).rev().collect::<Vec<i32>>();

    let mut used = vec![false; n as usize + 1];

    backtrack(&mut used, &mut ans, &nums, 0);

    return ans;
}

#[test]
fn tests() {
    for i in 1..=20 {
        construct_distanced_sequence(i);
    }
}
