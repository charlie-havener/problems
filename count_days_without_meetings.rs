pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
    let mut ans: i32 = 0;
    let mut end = 0;
    meetings.sort_unstable_by_key(|m| m[0]);
    for m in meetings {
        if m[0] > end {
            ans += m[0] - (end + 1);
        }
        end = end.max(m[1]);
    }
    ans += 0.max(days - end);
    return ans;
}

#[test]
fn tests() {
    let days = 10;
    let meetings = vec![vec![5,7],vec![1,3],vec![9,10]];
    assert_eq!(2, count_days(days, meetings));

    let days = 5;
    let meetings = vec![vec![2,4],vec![1,3]];
    assert_eq!(1, count_days(days, meetings));

    let days = 6;
    let meetings = vec![vec![1,6]];
    assert_eq!(0, count_days(days, meetings));
}
