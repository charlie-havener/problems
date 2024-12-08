pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {

    let mut dp: Vec<Vec<i32>> = vec![vec![0; events.len() + 1]; k as usize + 1];
    let mut s_idx: Vec<usize> = vec![events.len(); events.len() + 1];
    events.sort_unstable();

    for idx in 0..events.len() {
        s_idx[idx] = events.partition_point(|e| e[0] < events[idx][1] + 1);
    }

    for curr in (0..events.len()).rev() {
        for attend in 1..=k as usize {
            let nxt = s_idx[curr];
            //let nxt = events.partition_point(|e| e[0] < events[curr][1] + 1);
            let go = events[curr][2] + dp[attend-1][nxt as usize];
            let nogo = dp[attend][curr + 1];
            dp[attend][curr] = go.max(nogo);
        }
    }
    return dp[k as usize][0];
}

#[test]
fn tests() {
    let events = vec![vec![1,2,4],vec![3,4,3],vec![2,3,1]];
    let k = 2;
    assert_eq!(7, max_value(events, k));

    let events = vec![vec![1,2,4],vec![3,4,3],vec![2,3,10]];
    let k = 2;
    assert_eq!(10, max_value(events, k));

    let events = vec![vec![1,1,1],vec![2,2,2],vec![3,3,3],vec![4,4,4]];
    let k = 3;
    assert_eq!(9, max_value(events, k));
}
