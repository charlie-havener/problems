pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
    let mut ans = i32::MIN;
    let mut big = values[0];
    for (idx, v) in values[1..].iter().enumerate() {
        let idx = idx as i32 + 1;
        ans = ans.max(big + *v - idx as i32);
        big = big.max(*v + idx as i32);
    }
    return ans;
}

#[test]
fn tests() {
    let values = vec![8,1,5,2,6];
    assert_eq!(11, max_score_sightseeing_pair(values));

    let values = vec![1,2];
    assert_eq!(2, max_score_sightseeing_pair(values));
}
