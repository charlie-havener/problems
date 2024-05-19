pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
    
    let mut diffs = nums.iter().fold(Vec::with_capacity(nums.len()), |mut acc, v| {
        acc.push((v^k) - v);
        acc
    });
    diffs.sort_unstable_by_key(|v| -1 * v);

    let mut ans = nums.iter().map(|&i| i as i64).sum();

    for chunk in diffs.chunks_exact(2) {
        if chunk[0] + chunk[1] > 0 {
            ans += chunk[0] as i64 + chunk[1] as i64;
        }
    }
    return ans;

}

#[test]
fn tests() {
    let nums = vec![1,3,4,0,1,2,0];
    let k = 3;
    let edges = vec![vec![0,1],vec![0,2]];
    assert_eq!(21, maximum_value_sum(nums, k, edges));

    let nums = vec![1,2,1];
    let k = 3;
    let edges = vec![vec![0,1],vec![0,2]];
    assert_eq!(6, maximum_value_sum(nums, k, edges));

    let nums = vec![2,3];
    let k = 7;
    let edges = vec![vec![0,1],vec![0,2]];
    assert_eq!(9, maximum_value_sum(nums, k, edges));

    let nums = vec![7,7,7,7,7,7];
    let k = 3;
    let edges = vec![vec![0,1],vec![0,2]];
    assert_eq!(42, maximum_value_sum(nums, k, edges));
}
