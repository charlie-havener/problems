pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {

    let mut dp = Vec::with_capacity(questions.len());
    dp.push(0);

    for (idx, q)  in questions.iter().rev().enumerate() {
        let (s, b) = (q[0] as i64, q[1] as usize);
        if b < idx {
            dp.push(dp[idx].max(s + dp[idx - b]));
        } else {
            dp.push(dp[idx].max(s));
        }
    }
    return *dp.last().unwrap();
}

#[test]
fn tests() {
    let questions = vec![vec![3,2],vec![4,3],vec![4,4],vec![2,5]];
    assert_eq!(5, most_points(questions));

    let questions = vec![vec![1,1],vec![2,2],vec![3,3],vec![4,4],vec![5,5]];
    assert_eq!(7, most_points(questions));
}
