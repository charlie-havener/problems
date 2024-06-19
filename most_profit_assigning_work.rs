pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, mut worker: Vec<i32>) -> i32 {
    let l = difficulty.len();
    let mut jobs = (0..l).fold(Vec::with_capacity(l), |mut acc, idx| {
        acc.push((difficulty[idx], profit[idx]));
        acc
    });
    jobs.sort_unstable();

    worker.sort_unstable();
    let mut ans = 0;
    let mut large = 0;
    let mut curr = 0;

    for w in worker {
        while curr < l && jobs[curr].0 <= w {
            large = large.max(jobs[curr].1);
            curr += 1;
        }
        ans += large;
    }
    return ans;
}

#[test]
fn tests() {
    let d = vec![2,4,6,8,10];
    let p = vec![10,20,30,40,50];
    let w = vec![4,5,6,7];
    assert_eq!(100, max_profit_assignment(d,p,w));

    let d = vec![85,47,57];
    let p = vec![24,66,99];
    let w = vec![40,25,25];
    assert_eq!(0, max_profit_assignment(d,p,w));
}
