pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
    let mut ans = 0;

    let mut counts: Vec<i64> = vec![0; n as usize];
    for r in roads {
        counts[r[0] as usize] += 1;
        counts[r[1] as usize] += 1;
    }

    counts.sort_unstable();
    
    for i in 1..=n {
        ans += counts[i as usize - 1] * i as i64;
    }
    return ans;
}

#[test]
fn tests() {
    let n = 5;
    let r = vec![vec![0,1],vec![1,2],vec![2,3],vec![0,2],vec![1,3],vec![2,4]];
    assert_eq!(43, maximum_importance(n,r));

    let n = 5;
    let r = vec![vec![0,3],vec![2,4],vec![1,3]];
    assert_eq!(20, maximum_importance(n,r));
}
